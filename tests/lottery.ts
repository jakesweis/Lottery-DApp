import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Lottery } from "../target/types/lottery";
import { PublicKey } from '@solana/web3.js';
import { assert } from "chai";
import { BN } from "bn.js";

const MASTER_SEED = "MASTER_SEED";
const LOTTERY_SEED = "LOTTERY_SEED";
const TICKET_SEED  = "TICKET_SEED";

describe("lottery", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");

  const program = anchor.workspace.Lottery as Program<Lottery>;

  // Create accounts
  const payer = anchor.web3.Keypair.generate();
  const master = anchor.web3.Keypair.generate();
  const authority = anchor.web3.Keypair.generate();
  const buyer = anchor.web3.Keypair.generate();

  const lotteryIdBytes = Buffer.alloc(4); // Allocate a 4-byte buffer
  lotteryIdBytes.writeUInt32LE(1, 0);
  const ticketPrice = new BN(5);

  const lottery2IdBytes = Buffer.alloc(4); // Allocate a 4-byte buffer
  lottery2IdBytes.writeUInt32LE(2, 0);

  it("should initialize master", async () => {
    // Add funds to the payer account
    await airdrop(provider.connection, payer.publicKey);
    const [master_pkey, master_bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(MASTER_SEED),
      ], program.programId);

    // Call the initialize_master instruction
    await program.methods.initialize().accounts(
        {
          payer: payer.publicKey,
          master: master_pkey,
          systemProgram: anchor.web3.SystemProgram.programId
        }
    ).signers([payer]).rpc({ commitment: "confirmed" })
  
    let masterData = await program.account.master.fetch(master_pkey);

    assert.strictEqual(masterData.lastId, 0)
  });

  it("should create lottery", async () => {
    await airdrop(provider.connection, authority.publicKey);
    const [master_pkey, master_bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(MASTER_SEED),
      ], program.programId);
    const [lottery_pkey, lottery_bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(LOTTERY_SEED),
        lotteryIdBytes,
      ], program.programId);

    await program.methods.newLottery(ticketPrice).accounts(
        {
          authority: authority.publicKey,
          lottery: lottery_pkey,
          master: master_pkey,
          systemProgram: anchor.web3.SystemProgram.programId
        }
    ).signers([authority]).rpc({ commitment: "confirmed" })
  
    let lotteryData = await program.account.lottery.fetch(lottery_pkey);

    assert.strictEqual(lotteryData.id, 1);
    assert.strictEqual(lotteryData.authority.toString(), authority.publicKey.toString());
    assert.ok(lotteryData.ticketPrice.eq(ticketPrice));
    assert.strictEqual(lotteryData.lastTicketId, 0);
    assert.strictEqual(lotteryData.winnerId, null)
    assert.strictEqual(lotteryData.claimed, false)
  });

  it("should be able to buy ticket", async () => {
    await airdrop(provider.connection, buyer.publicKey);
    const [lottery_pkey, lottery_bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(LOTTERY_SEED),
        lotteryIdBytes,
      ], program.programId);
    const [ticket_pkey, ticket_bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(TICKET_SEED),
        lottery_pkey.toBuffer(),
        lotteryIdBytes,
      ], program.programId);

    await program.methods.purchaseTicket(1).accounts(
        {
          lottery: lottery_pkey,
          buyer: buyer.publicKey,
          ticket: ticket_pkey,
          systemProgram: anchor.web3.SystemProgram.programId
        }
    ).signers([buyer]).rpc({ commitment: "confirmed" })
  
    let ticketData = await program.account.ticket.fetch(ticket_pkey);

    assert.strictEqual(ticketData.id, 1);
    assert.strictEqual(ticketData.authority.toString(), buyer.publicKey.toString());
    assert.strictEqual(ticketData.lotteryId, 1)
  });

  it("try to pick winner, but no tickets have been bought", async () => {
    await airdrop(provider.connection, authority.publicKey);
    const [master_pkey, master_bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(MASTER_SEED),
      ], program.programId);
    const [lottery_pkey, lottery_bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(LOTTERY_SEED),
        lottery2IdBytes,
      ], program.programId);

    await program.methods.newLottery(ticketPrice).accounts(
      {
        authority: authority.publicKey,
        lottery: lottery_pkey,
        master: master_pkey,
        systemProgram: anchor.web3.SystemProgram.programId
      }
    ).signers([authority]).rpc({ commitment: "confirmed" })

    let should_fail = "This Should Fail"

    try{
      await program.methods.chooseWinner(2).accounts(
        {
          lottery: lottery_pkey,
          authority: authority.publicKey,
        }
      ).signers([authority]).rpc({ commitment: "confirmed" })
    } catch (error) {
      const err = anchor.AnchorError.parse(error.logs);
      assert.strictEqual(err.error.errorCode.code, "NoTickets");
      should_fail = "Failed";
    }
    assert.strictEqual(should_fail, "Failed");
  });

  it("claim prize with winning ticket id", async () => {
    await airdrop(provider.connection, authority.publicKey);
    const [lottery_pkey, lottery_bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(LOTTERY_SEED),
        lotteryIdBytes,
      ], program.programId);
    const [ticket_pkey, ticket_bump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(TICKET_SEED),
        lottery_pkey.toBuffer(),
        lottery2IdBytes,
      ], program.programId);

    await program.methods.purchaseTicket(1).accounts(
      {
        lottery: lottery_pkey,
        buyer: buyer.publicKey,
        ticket: ticket_pkey,
        systemProgram: anchor.web3.SystemProgram.programId
      }
    ).signers([buyer]).rpc({ commitment: "confirmed" })
  
    await program.methods.chooseWinner(1).accounts(
      {
        lottery: lottery_pkey,
        authority: authority.publicKey,
      }
    ).signers([authority]).rpc({ commitment: "confirmed" })

    await program.methods.prizeClaim(1, 2).accounts(
      {
        authority: authority.publicKey,
        lottery: lottery_pkey,
        ticket: ticket_pkey,
        systemProgram: anchor.web3.SystemProgram.programId
      }
    ).signers([authority]).rpc({ commitment: "confirmed" })

    let lotteryData = await program.account.lottery.fetch(lottery_pkey);
    let ticketData = await program.account.ticket.fetch(ticket_pkey);

    assert.strictEqual(lotteryData.claimed, true);
  });
});

async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}