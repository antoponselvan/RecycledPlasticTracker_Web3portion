import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { RePlasticTracker } from "../target/types/re_plastic_tracker";
import { PublicKey, SystemProgram} from "@solana/web3.js";
import {assert, use} from "chai"

describe("re-plastic-tracker", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.RePlasticTracker as Program<RePlasticTracker>;
  const user = anchor.web3.Keypair.generate()
  let productAccount: PublicKey;
  let reviewAccount: PublicKey;
  let serialNum: string
  console.log(user.publicKey.toString())
  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Confirm airdrop", async () => {
    const airdropSellerSign = await provider.connection.requestAirdrop(
      user.publicKey,
      1e9
    );
    const latestSellerBlockhash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestSellerBlockhash.blockhash,
      lastValidBlockHeight: latestSellerBlockhash.lastValidBlockHeight,
      signature: airdropSellerSign,
    });
  })

  it("Confirm Product Account Creation", async () => {
    serialNum = "AAAA1113" 
    let variant = 0
    let rePlasticPct = 99     
    let ingridientManufacturerKey = "AFhu69Fx9G6VstaqnqsPyp6maKsmjMZx1wdqKQi4swfm"
    let purchaserKey = "CBjFUzrHh69d8ejgYkdEBet3srjHhHekHgtKNs7xmPsX"
    let ingridientSerialNum = "AAAA1112"
    let randomVar = "Asdad";

    [productAccount] = await PublicKey.findProgramAddress(
      [
        user.publicKey.toBytes(),
        Buffer.from(anchor.utils.bytes.utf8.encode(serialNum))
      ],
      program.programId
    )

    await program.methods
    .addOrUpdateProduct(variant, rePlasticPct, serialNum, ingridientManufacturerKey, ingridientSerialNum,purchaserKey)
    .accounts({
      initializer: user.publicKey,
      pdaAccount: productAccount,
      systemProgram: SystemProgram.programId
    }) 
    .signers([user])
    .rpc(); 
    // [reviewAccount] = await PublicKey.findProgramAddress(
    //   [
    //     user.publicKey.toBytes(),
    //     Buffer.from(anchor.utils.bytes.utf8.encode(serialNum)),
    //   ],
    //   program.programId
    // );

    const storedProductAccount = await program.account.productAccountState.fetch(productAccount);
    console.log(storedProductAccount)
    assert.equal(storedProductAccount.serialNum, serialNum)
  
  })

});
