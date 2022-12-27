import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey } from '@solana/web3.js';
import { HelloAnchor } from "../target/types/hello_anchor";

describe("hello-anchor", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.HelloAnchor as Program<HelloAnchor>;

    it("Is initialized!", async () => {
        const [pda, bump] = await PublicKey.findProgramAddress(
            [Buffer.from("counter-seeds")],
            program.programId,
        )
        console.log("pda", pda.toBase58(), ", bump", bump);
        // Add your test here.
        const tx = await program.methods.initialize().accounts({
            funding: provider.wallet.publicKey,
            counter: pda,
        }).rpc();

        console.log("Your transaction signature", tx);
        const tx2 = await program.methods.incr(1).accounts({
            funding: provider.wallet.publicKey,
            counter: pda,
        }).rpc();
        console.log("Your transaction signature", tx2);

        const tx3 = await program.methods.incr(2).accounts({
            funding: provider.wallet.publicKey,
            counter: pda,
        }).rpc();
        console.log("Your transaction signature", tx3);
    });
});
