# Scout: Security Analysis Tool

<p align="center">
  <img src="https://raw.githubusercontent.com/CoinFabrik/scout/c1eb3073f85b051dc9ce2fa0ab1ebab4bde0914e/assets/scout.png" alt="Scout in a Dark Forest" width="300" center  />
</p>

Scout is an extensible open-source tool intended to assist [ink!](https://use.ink/smart-contracts-polkadot/) and [Soroboan](https://stellar.org/soroban) smart contract developers and auditors detect common security issues and deviations from best practices.

This tool will help developers write secure and more robust smart contracts.

Our interest in this project comes from our experience in manual auditing and our usage of comparable tools in other blockchains. To improve coverage and precision, we´ll persist in research efforts on static and dynamic analysis techniques.

## Quick Start

For a quick start, make sure that [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) is installed on your computer. Then, install Scout dependencies by running the following command:

```bash
cargo install cargo-dylint dylint-link
```

Afterwards, install Scout with the following command:

```bash
cargo install cargo-scout-audit
```

To run Scout on your project, navigate to its root directory and execute the following command:

```bash
cargo scout-audit
```
For more information on Scout's installation and usage, please refer to Scout's documentation for [ink!](https://github.com/CoinFabrik/scout) or [Soroban](https://github.com/CoinFabrik/scout-soroban).

## How to integrate with Github Actions

Create `.github/workflows/scout.yml`:

```yaml
name: Scout-actions
on: [push]
jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: coinfabrik/scout-actions@v1
        with:
          target: './target/Cargo.toml'
```


### YML Description

- **name: Scout-actions**: This is the name of the GitHub Action which will be viewed on the GitHub Actions dashboard.
- **on: [push]**: This line specifies the event that will trigger the action. In this case, the GitHub Action fires whenever a push is made to the repository.
- **jobs**: GitHub Actions can contain several jobs. In this case, only one job named analyze has been set up.
- **analyze**: This is the name of the job.
- **runs-on: ubuntu-latest**: This specifies the runtime environment for the job. Here, the job will run on the latest available Ubuntu version.
- **steps**: This is a list of tasks to be carried out in the job. In this case, there are two tasks.
- **uses: actions/checkout@v2**: The first task uses a GitHub Action called 'checkout@v2'. This is a predefined Action that allows GitHub Actions to work with a copy of your repository.
- **uses: coinfabrik/scout-actions@v1**: The second task uses the GitHub Action 'scout-actions@v1', a version specified by coinfabrik.
- **with** and **target: './target/Cargo.toml'**: Under the 'coinfabrik/scout-actions@v1' task, an additional option with is configured, which sets a specific target for the action under with. In this case, the target is the file './target/Cargo.toml'.
This toml file in the target directory likely has the dependencies and project configuration that will undergo analysis.
In short, this .yml file sets up a GitHub Action that activates on any push to the repository. When triggered, it will checkout the repository and then run the 'scout-actions@v1' Action on the './target/Cargo.toml' file.




### Options

| Key              | Description
|------------------|------------
| `target`         | The path to the root of the project to be analyzed by Scout. It can be a path of Cargo.toml, and it defaults to the repo root.


## Detectors

Detectors available for Scout Actions are the ones available for Scout in its [ink!](https://github.com/CoinFabrik/scout?tab=readme-ov-file#detectors) and [Soroban](https://github.com/CoinFabrik/scout-soroban?tab=readme-ov-file#detectors) versions.                                                                                              

## Acknowledgements

Scout is an open source vulnerability analyzer developed by [CoinFabrik's](https://www.coinfabrik.com/) Research and Development team.

We received support through grants from both the [Web3 Foundation Grants Program](https://github.com/w3f/Grants-Program/tree/master), the [Aleph Zero Ecosystem Funding Program](https://alephzero.org/ecosystem-funding-program) and the [Stellar Community Fund](https://communityfund.stellar.org).

| Grant Program | Description |
|---------------|-------------|
| ![Web3 Foundation](https://raw.githubusercontent.com/CoinFabrik/scout/main/assets/web3-foundation.png) | **Proof of Concept:** We collaborated with the [Laboratory on Foundations and Tools for Software Engineering (LaFHIS)](https://lafhis.dc.uba.ar/) at the [University of Buenos Aires](https://www.uba.ar/internacionales/index.php?lang=en) to establish analysis techniques and tools for our detectors, as well as to create an initial list of vulnerability classes and code examples. [View Grant](https://github.com/CoinFabrik/web3-grant) \| [Application Form](https://github.com/w3f/Grants-Program/blob/master/applications/ScoutCoinFabrik.md).<br><br>**Prototype:** We built a functioning prototype using linting detectors built with [Dylint](https://github.com/trailofbits/dylint) and expanded the list of vulnerability classes, detectors, and test cases. [View Prototype](https://coinfabrik.github.io/scout/) \| [Application Form](https://github.com/w3f/Grants-Program/blob/master/applications/ScoutCoinFabrik_2.md). |
| ![Aleph Zero](https://raw.githubusercontent.com/CoinFabrik/scout/main/assets/aleph-zero.png) | We improved the precision and number of detectors for the tool with a multi-phase approach. This included a manual vulnerability analysis of projects within the Aleph Zero ecosystem, comprehensive testing of the tool on leading projects, and refining its detection accuracy. |
| ![Stellar Community Fund](https://github.com/CoinFabrik/scout-soroban/blob/main/docs/static/img/stellar.png) | We added support for Stellar's smart contract language, Soroban. We included various output formats, such as an HTML report, improved the tool's precision and recall, and added a GitHub action to run the tool with pull requests.|

## About CoinFabrik

We - [CoinFabrik](https://www.coinfabrik.com/) - are a research and development company specialized in Web3, with a strong background in cybersecurity. Founded in 2014, we have worked on over 180 blockchain-related projects, EVM based and also for Solana, Algorand, Stellar and Polkadot. Beyond development, we offer security audits through a dedicated in-house team of senior cybersecurity professionals, currently working on code in Substrate, Solidity, Clarity, Rust, TEAL and Stellar Soroban.

Our team has an academic background in computer science and mathematics, with work experience focused on cybersecurity and software development, including academic publications, patents turned into products, and conference presentations. Furthermore, we have an ongoing collaboration on knowledge transfer and open-source projects with the University of Buenos Aires.

## License

Scout is licensed and distributed under a MIT license. [Contact us](https://www.coinfabrik.com/) if you're looking for an exception to the terms.
