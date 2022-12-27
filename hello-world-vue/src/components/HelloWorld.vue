<script>
import * as web3 from "@solana/web3.js";
import { Buffer } from "buffer";
import { u32 } from "@solana/buffer-layout";

export default {
  name: 'HelloWorld',
  data() {
    return {

    };
  },
  methods: {
    async handleClick() {
      const secretKey = Uint8Array.from([39,200,223,129,7,187,231,0,60,139,213,134,61,123,105,106,105,183,119,59,4,187,86,82,218,182,26,220,232,117,148,39,127,193,26,147,18,125,138,125,102,27,211,194,53,168,55,93,110,138,37,182,78,146,199,46,134,250,130,223,48,218,61,202]);
      const keypair = web3.Keypair.fromSecretKey(secretKey);
      console.log(keypair.publicKey.toBase58());
      const programId = new web3.PublicKey("Cj9VjDrohXuz914FNbaTMmEqYXkLkJKZn9FCq4UGGGwr");

      const [pda, bump] = web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("counter-seeds"),
        ],
        programId,
      );
      console.log("pda", pda.toBase58(), ", bump", bump);

      const connection = new web3.Connection(web3.clusterApiUrl("testnet"));

      const instruction_t = u32();
      const insData = Buffer.alloc(instruction_t.span);

      instruction_t.encode(1, insData);
      const bumpData = Buffer.from(Uint8Array.of(bump));
      const data = Buffer.concat([insData, bumpData], insData.length + bumpData.length);
      console.log("data", data);
      const tx = new web3.Transaction();
      tx.add(
        new web3.TransactionInstruction({
          keys: [
            {pubkey: keypair.publicKey, isSigner: true, isWritable: true},
            {pubkey: pda, isSigner: false, isWritable: true},
            {pubkey: web3.SystemProgram.programId, isSigner: false, isWritable: false},
          ],
          programId: programId,
          data,
        }),
      );
      const result = await web3.sendAndConfirmTransaction(connection, tx, [keypair]);
      console.log(result);
    },
  }
}
</script>

<template>
  <div class="greetings">
    <button @click="handleClick">Click Me!</button>
  </div>
</template>

<style scoped>
h1 {
  font-weight: 500;
  font-size: 2.6rem;
  top: -10px;
}

h3 {
  font-size: 1.2rem;
}

.greetings h1,
.greetings h3 {
  text-align: center;
}

@media (min-width: 1024px) {
  .greetings h1,
  .greetings h3 {
    text-align: left;
  }
}
</style>
