// End-to-end test of the Surveys contract on a LOCAL throwaway Geode dev node.
// Usage: node e2e.mjs <path to target/ink dir>
import fs from 'node:fs';
import path from 'node:path';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { ContractPromise, BlueprintPromise, Abi } from '@polkadot/api-contract';
import { stringToU8a, u8aToHex, BN } from '@polkadot/util';

const URL = 'ws://127.0.0.1:9989';
const DIR = process.argv[2];
const abiJson = JSON.parse(fs.readFileSync(path.join(DIR, 'surveys.json'), 'utf8'));
const wasm = fs.readFileSync(path.join(DIR, 'surveys.wasm'));

let pass = 0, fail = 0;
const results = [];
function check(name, ok, detail = '') {
  if (ok) pass++; else fail++;
  const line = `${ok ? 'PASS' : 'FAIL'}  ${name}${detail ? '  (' + detail + ')' : ''}`;
  results.push(line); console.log(line);
}
const b = (s) => u8aToHex(stringToU8a(s));

const api = await ApiPromise.create({ provider: new WsProvider(URL), noInitWarn: true });
const chain = (await api.rpc.system.chain()).toString();
const peers = (await api.rpc.system.peers()).length;
if (chain !== 'Development' || peers !== 0) {
  console.error(`REFUSING: chain=${chain} peers=${peers} (expected a lone Development node)`);
  process.exit(2);
}
const spec = api.runtimeVersion.specVersion.toString();
console.log(`chain=${chain} peers=${peers} specVersion=${spec}`);

const kr = new Keyring({ type: 'sr25519' });
const [alice, bob, charlie, dave, eve, ferdie] =
  ['Alice', 'Bob', 'Charlie', 'Dave', 'Eve', 'Ferdie'].map((n) => kr.addFromUri('//' + n));
const name = new Map([[alice.address, 'alice'], [bob.address, 'bob'], [charlie.address, 'charlie'],
  [dave.address, 'dave'], [eve.address, 'eve'], [ferdie.address, 'ferdie']]);

const ED = api.consts.balances.existentialDeposit.toBn();
const P = BN.max(ED.muln(10), new BN('1000000000000')); // payout per response
console.log(`ED=${ED} payout=${P}`);

// submit an extrinsic and wait until it is in a block; returns {ok, events, dispatchError}
function send(tx, signer) {
  return new Promise((resolve, reject) => {
    tx.signAndSend(signer, ({ status, events, dispatchError }) => {
      if (status.isInBlock) {
        let err = null;
        if (dispatchError) {
          if (dispatchError.isModule) {
            const d = api.registry.findMetaError(dispatchError.asModule);
            err = `${d.section}.${d.name}`;
          } else err = dispatchError.toString();
        }
        resolve({ ok: !dispatchError, events, err });
      }
    }).catch(reject);
  });
}

const BIG = api.registry.createType('WeightV2', { refTime: new BN('500000000000'), proofSize: new BN('5000000') });

// ---- deploy: upload (TC member) then instantiate from the code hash ----
const up = await send(api.tx.contracts.uploadCode(u8aToHex(wasm), null, 'Deterministic'), alice);
check('upload code as TC member (Alice)', up.ok, up.err || '');
const stored = up.events.find(({ event }) => api.events.contracts.CodeStored.is(event));
const codeHash = stored.event.data[0].toHex();
console.log('codeHash', codeHash);

const bp = new BlueprintPromise(api, abiJson, codeHash);
let contractAddr;
await new Promise((resolve, reject) => {
  bp.tx.new({ gasLimit: BIG, storageDepositLimit: null }).signAndSend(alice, (r) => {
    if (r.status.isInBlock) {
      if (r.dispatchError) reject(new Error('instantiate failed ' + r.dispatchError.toString()));
      contractAddr = r.contract.address.toString();
      resolve();
    }
  }).catch(reject);
});
check('instantiate as Alice (becomes root)', !!contractAddr, contractAddr);
const c = new ContractPromise(api, abiJson, contractAddr);

const free = async (a) => (await api.query.system.account(a)).data.free.toBn();
const contractStart = await free(contractAddr);

// dry-run a message; returns the decoded human output, e.g. {Ok: {Err: 'ContractPaused'}} or {Ok: {Ok: null}}
async function q(msg, signer, value, ...args) {
  const { result, output } = await c.query[msg](signer.address, { gasLimit: BIG, storageDepositLimit: null, value }, ...args);
  if (result.isErr) return { dispatchErr: result.asErr.toString() };
  return output ? output.toJSON() : null;
}
// run a message for real (dry run first for the gas); returns {ok, err, transfers: [{to, amount}]}
async function tx(msg, signer, value, ...args) {
  const dry = await c.query[msg](signer.address, { gasLimit: BIG, storageDepositLimit: null, value }, ...args);
  const gas = api.registry.createType('WeightV2', {
    refTime: dry.gasRequired.refTime.toBn().muln(2), proofSize: dry.gasRequired.proofSize.toBn().muln(2) });
  const r = await send(c.tx[msg]({ gasLimit: gas, storageDepositLimit: null, value }, ...args), signer);
  const transfers = r.events
    .filter(({ event }) => api.events.balances.Transfer.is(event) && event.data[0].toString() === contractAddr)
    .map(({ event }) => ({ to: event.data[1].toString(), amount: event.data[2].toBn() }));
  return { ok: r.ok, err: r.err, transfers, dry: dry.output?.toJSON() };
}
const isErr = (out, e) => JSON.stringify(out) === JSON.stringify({ ok: { err: e } });
const isOk = (out) => out && out.ok && 'ok' in out.ok;
const paidTo = (t, who, amt) => t.transfers.length === 1 && t.transfers[0].to === who.address && t.transfers[0].amount.eq(amt);

// ---- set up surveys ----
let staked = new BN(0);
async function make(len, who, nm, max) {
  const stake = P.muln(max);
  const qs = { 1: [b('q1')], 3: [b('q1'), b('q2'), b('q3')], 5: [b('q1'), b('q2'), b('q3'), b('q4'), b('q5')] }[len];
  staked = staked.add(stake);
  const r = await tx(`makeANewSurvey${len}`, who, stake, b(nm), true, P, max, ...qs);
  check(`${name.get(who.address)} makes ${len}Q survey "${nm}" (stake ${max} x payout)`, r.ok && isOk(r.dry), r.err || '');
  const mine = (await q('getMySurveys', who, 0));
  const list = mine.ok[`surveys${len}`];
  return list[list.length - 1].surveyId;
}
const totalFree = async (a) => { const d = (await api.query.system.account(a)).data; return d.free.toBn().add(d.reserved.toBn()); };
const s1a = await make(1, bob, 'bob-1a', 3);
const s1b = await make(1, bob, 'bob-1b', 3);
const s3 = await make(3, bob, 'bob-3', 2);
const s5 = await make(5, bob, 'bob-5', 2);
const c1 = await make(1, charlie, 'charlie-1', 3);

const take = (len, who, id, ip) => {
  const ans = { 1: [b('a1')], 3: [b('a1'), b('a2'), b('a3')], 5: [b('a1'), b('a2'), b('a3'), b('a4'), b('a5')] }[len];
  return tx(`takeASurvey${len}`, who, 0, id, b(ip), ...ans);
};
const takeQ = (len, who, id, ip) => {
  const ans = { 1: [b('a1')], 3: [b('a1'), b('a2'), b('a3')], 5: [b('a1'), b('a2'), b('a3'), b('a4'), b('a5')] }[len];
  return q(`takeASurvey${len}`, who, 0, id, b(ip), ...ans);
};

const heldAfterSetup = await free(contractAddr);
check('contract holds every stake after setup', heldAfterSetup.sub(contractStart).eq(staked), `held ${heldAfterSetup} staked ${staked}`);
let r = await take(1, dave, s1a, '10.0.0.1');
check('dave takes bob-1a from ip 10.0.0.1 and is paid the payout', r.ok && paidTo(r, dave, P));
r = await take(3, dave, s3, '10.0.0.9');
check('dave takes bob-3 (3Q) and is paid', r.ok && paidTo(r, dave, P));
r = await take(5, dave, s5, '10.0.0.9');
check('dave takes bob-5 (5Q) and is paid', r.ok && paidTo(r, dave, P));

// ---- (1) per-owner IP ban ----
r = await tx('banThisIpAddress', charlie, 0, s1a, b('10.0.0.1'));
let o = await takeQ(1, eve, s1b, '10.0.0.1');
check('charlie cannot ban an ip on bob\'s survey (no effect)', r.ok && isOk(o), JSON.stringify(o));
r = await tx('banThisIpAddress', bob, 0, s1a, b('10.0.0.1'));
check('bob bans ip 10.0.0.1 (it took bob-1a)', r.ok && isOk(r.dry));
o = await takeQ(1, eve, s1b, '10.0.0.1');
check('banned ip blocked from bob\'s OTHER survey bob-1b (AccessDenied)', isErr(o, 'AccessDenied'), JSON.stringify(o));
r = await take(1, eve, s1b, '10.0.0.1');
check('  ...and a real submission is rejected with no payout', !r.ok && r.transfers.length === 0, r.err || '');
r = await take(1, eve, c1, '10.0.0.1');
check('banned ip can still take charlie\'s survey and is paid', r.ok && paidTo(r, eve, P));
r = await take(1, eve, s1b, '10.0.0.2');
check('a different ip can still take bob-1b', r.ok && paidTo(r, eve, P));

// ---- (2) pause ----
o = await q('pauseContract', bob, 0, true);
check('bob cannot pause (NotRoot)', isErr(o, 'NotRoot'), JSON.stringify(o));
r = await tx('pauseContract', alice, 0, true);
check('alice (root) pauses', r.ok && isOk(r.dry));
const blocked = async (label, out, e) => check(`${e === 'ContractPaused' ? 'paused' : 'shut down'}: ${label} -> ${e}`, isErr(out, e), JSON.stringify(out));
const allWrites = async (e) => {
  await blocked('make 1Q', await q('makeANewSurvey1', bob, P.muln(2), b('x'), true, P, 2, b('q')), e);
  await blocked('make 3Q', await q('makeANewSurvey3', bob, P.muln(2), b('x'), true, P, 2, b('q'), b('q'), b('q')), e);
  await blocked('make 5Q', await q('makeANewSurvey5', bob, P.muln(2), b('x'), true, P, 2, b('q'), b('q'), b('q'), b('q'), b('q')), e);
  await blocked('take 1Q', await takeQ(1, ferdie, c1, '10.0.0.7'), e);
  await blocked('take 3Q', await takeQ(3, ferdie, s3, '10.0.0.7'), e);
  await blocked('take 5Q', await takeQ(5, ferdie, s5, '10.0.0.7'), e);
  await blocked('ban', await q('banThisIpAddress', bob, 0, s1a, b('10.0.0.1')), e);
};
await allWrites('ContractPaused');
await blocked('close my survey', await q('closeMySurvey', bob, 0, s1a, 1), 'ContractPaused');
r = await take(1, ferdie, c1, '10.0.0.7');
check('paused: real take submission rejected, no payout', !r.ok && r.transfers.length === 0, r.err || '');
o = await q('getAvailableSurveys', ferdie, 0);
check('paused: getAvailableSurveys still works', o && o.ok && o.ok.surveys1.length >= 1, `${o?.ok?.surveys1?.length} 1Q listed`);
o = await q('getSurveyResponses1', bob, 0, s1a, 0, 1);
check('paused: owner can still read responses', o && o.ok && o.ok.length === 1);
o = await q('getSurveyById', ferdie, 0, s3);
check('paused: getSurveyById still works', o && o.ok && o.ok.surveys3.length === 1);
r = await tx('pauseContract', alice, 0, false);
check('alice unpauses', r.ok && isOk(r.dry));
r = await take(1, ferdie, c1, '10.0.0.7');
check('after unpause, ferdie takes charlie-1 and is paid', r.ok && paidTo(r, ferdie, P));

// ---- (3) shutdown ----
o = await q('shutdownContract', bob, 0);
check('bob cannot shut down (NotRoot)', isErr(o, 'NotRoot'), JSON.stringify(o));
r = await tx('pauseContract', alice, 0, true);
r = await tx('shutdownContract', alice, 0);
check('alice shuts down (while paused)', r.ok && isOk(r.dry) && r.transfers.length === 0, 'no coin moved by shutdown');
await allWrites('ContractShutDown');
o = await q('pauseContract', alice, 0, false);
check('shut down: root cannot unpause', isErr(o, 'ContractShutDown'), JSON.stringify(o));
o = await q('pauseContract', alice, 0, true);
check('shut down: root cannot pause either', isErr(o, 'ContractShutDown'), JSON.stringify(o));
o = await q('shutdownContract', alice, 0);
check('shut down: calling shutdown again is harmless', isOk(o), JSON.stringify(o));
o = await q('getAvailableSurveys', ferdie, 0);
check('shut down: available list is empty',
  o && o.ok && !o.ok.surveys1.length && !o.ok.surveys3.length && !o.ok.surveys5.length);
o = await q('getSurveyResponses1', bob, 0, s1a, 0, 1);
check('shut down: owner can still read responses', o && o.ok && o.ok.length === 1);

// owners close and get exactly their leftover balance back
const balOf = async (who, id, len) => {
  const s = (await q('getSurveyById', who, 0, id)).ok[`surveys${len}`][0];
  return new BN(String(s.surveyBalance).replace(/^0x/, ''), String(s.surveyBalance).startsWith('0x') ? 16 : 10);
};
o = await q('closeMySurvey', eve, 0, s1a, 1);
check('shut down: non-owner cannot close bob\'s survey (AccessDenied)', isErr(o, 'AccessDenied'), JSON.stringify(o));
let refunded = new BN(0);
for (const [who, id, len, nm] of [[bob, s1a, 1, 'bob-1a'], [bob, s1b, 1, 'bob-1b'], [bob, s3, 3, 'bob-3'],
  [bob, s5, 5, 'bob-5'], [charlie, c1, 1, 'charlie-1']]) {
  const left = await balOf(who, id, len);
  r = await tx('closeMySurvey', who, 0, id, len);
  const s = (await q('getSurveyById', who, 0, id)).ok[`surveys${len}`][0];
  check(`shut down: ${name.get(who.address)} closes ${nm} and gets its leftover back (${left})`,
    r.ok && paidTo(r, who, left) && s.open === false && new BN(String(s.surveyBalance)).isZero());
  refunded = refunded.add(left);
  r = await tx('closeMySurvey', who, 0, id, len);
  check(`  ...closing ${nm} again refunds nothing`, r.ok && r.transfers.length === 0);
}
const responsesPaid = P.muln(6); // dave x3, eve x2, ferdie x1
check('refunds = stakes - payouts (no coin lost or stuck)', refunded.eq(staked.sub(responsesPaid)), `staked ${staked} paid ${responsesPaid} refunded ${refunded}`);
const contractEnd = await free(contractAddr);
check('contract balance is back to what it held right after instantiation',
  contractEnd.eq(contractStart), `start ${contractStart} end ${contractEnd}`);

console.log(`\n${pass} passed, ${fail} failed`);
fs.writeFileSync(path.join(process.cwd(), 'e2e-results.txt'),
  `chain=${chain} spec=${spec} contract=${contractAddr} codeHash=${codeHash}\n` + results.join('\n') + `\n${pass} passed, ${fail} failed\n`);
await api.disconnect();
process.exit(fail ? 1 : 0);
