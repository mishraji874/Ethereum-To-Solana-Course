import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import fs from 'fs'
let idl = JSON.parse(fs.readFileSync('target/idl/day_5.json', 'utf-8'))

describe("day5", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Change this to be your programID
  const programID = "ALKKgt9LJuf7RzyNh2B6dVjwBh7eWC8DAu1T3dRvoic7";
  const program = new Program(idl, programID, anchor.getProvider());

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

