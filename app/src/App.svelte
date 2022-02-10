<script lang="ts">
  import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
  import {
    LedgerWalletAdapter,
    PhantomWalletAdapter,
    SlopeWalletAdapter,
    SolflareWalletAdapter,
    SolletExtensionWalletAdapter,
    SolletWalletAdapter,
    TorusWalletAdapter,
  } from "@solana/wallet-adapter-wallets";
  export let name: string;
</script>

<main>
  <div class="header">
    <div class="link-items">
      <a href="/" class="link-item"> Dashboard </a>
    </div>
    <button class="connect-wallet-btn">Connect</button>
  </div>
  <div class="content" />
  <div class="footer" />
</main>

<style>
  :global(body) {
    background-color: rgb(12, 30, 127);
  }
  :global(p) {
    color: white;
  }
  :global(a) {
    color: rgb(255, 0, 142);
  }

  .header {
    display: flex;
    justify-content: space-between;
  }
  .link-items {
    display: flex;
    justify-content: flex-start;
  }
  .link-item {
    color: white;
  }
  .connect-wallet-btn {
    background-color: rgb(255, 0, 142);
    color: wheat;
    border: none;
    border-radius: 10px;
    padding-top: 10px;
    padding-bottom: 10px;
    padding-left: 20px;
    padding-right: 20px;
  }
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
