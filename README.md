# Scout: Security Analysis Tool - Github action

<p align="center">
  <img src="https://raw.githubusercontent.com/CoinFabrik/scout/c1eb3073f85b051dc9ce2fa0ab1ebab4bde0914e/assets/scout.png" alt="Scout in a Dark Forest" width="300" center  />
</p>

Scout is an extensible open-source tool intended to assist [ink!](https://use.ink/smart-contracts-polkadot/), [Soroban](https://stellar.org/soroban) and [Substrate](https://substrate.io/) developers and auditors detect common security issues and deviations from best practices.

## How to integrate with Github Actions

Create `.github/workflows/scout.yml`:

```yaml
name: Scout-actions
on: [push]
jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: coinfabrik/scout-actions@v3
        with:
          target: './target'
          scout_args: (scout arguments)
          scout_extra_args: (scout extra arguments)
```


### YML Description

- **name: Scout-actions**: This is the name of the GitHub Action which will be viewed on the GitHub Actions dashboard.
- **on: [push]**: This line specifies the event that will trigger the action. In this case, the GitHub Action fires whenever a push is made to the repository.
- **jobs**: GitHub Actions can contain several jobs. In this case, only one job named analyze has been set up.
- **analyze**: This is the name of the job.
- **runs-on: ubuntu-latest**: This specifies the runtime environment for the job. Here, the job will run on the latest available Ubuntu version.
- **steps**: This is a list of tasks to be carried out in the job. In this case, there are two tasks.
- **uses: actions/checkout@v4**: The first task uses a GitHub Action called 'checkout@v4'. This is a predefined Action that allows GitHub Actions to work with a copy of your repository.
- **uses: coinfabrik/scout-actions@v3**: The second task uses the GitHub Action 'scout-actions@v3', a version specified by coinfabrik.
- **with** and **target**: **'./target'**: Under the 'coinfabrik/scout-actions@v3' task, an additional option 
with is configured, which sets a specific target for the action under with. In this case, the target is the path `./target`, and scout will run on sub/project: `./target/Cargo.toml`.
This toml file in the target directory likely has the dependencies and project configuration that will undergo analysis.
- **with/scout_args**: allows you to overwrite current arguments for scout, which make it generate a markdown report which can be later included in PRs and tickets. Most users don't need to change this.
- **with/scout_extra_args**: allows you to specify extra arguments for scout, which will be added to the already present for markdown report generation. Most users don't need to change this.

 
In short, this .yml file sets up a GitHub Action that activates on any push to the repository. When triggered, it will checkout the repository and then run the 'scout-actions@v3' Action on the './target' path.


### Options

| Key                | Description                                                                                                                    |
|--------------------|--------------------------------------------------------------------------------------------------------------------------------|
| `target`           | The path to the root of the project to be analyzed by Scout. It can be a path of `Cargo.toml`, and it defaults to the repo root. | 
| `scout_args`       | Allows you to overwrite the arguments for scout. The default makes scout output a markdown output.                             | 
| `scout_extra_args` | This parameter allows you to add arguments for scout execution while keeping the required for markdown output.                 | 


## Detectors

Refer to Scout's [documentation site](https://coinfabrik.github.io/scout-audit/docs/intro) for a full list of the detectors for Ink, Soroban and Substrate.

## Full example

```yaml
name: scout-audit
on:
  pull_request:
    branches:
      - main

jobs:
  scout-audit:
    runs-on: ubuntu-latest
    permissions:
      pull-requests: write
      contents: write
      repository-projects: write
    steps:
      - name: checkout
        uses: actions/checkout@v4

      - name: do scout
        uses: coinfabrik/scout-actions@v3.1
        with:
          target: '' # Path of the project/file to execute Scout on.

      - uses: mshick/add-pr-comment@v2.8.2
        with:
          message-path:  ${{ github.workspace }}/report.md

      # # Optional: Add the following step to block the merge of the commit if Scout finds any issues.
      - name: Check for error
        run: |
          if [ -f "${{ github.workspace }}/FAIL" ]; then
            echo "Error: Scout found issues."
            exit 1
          fi
```


## Acknowledgements

Scout is an open source vulnerability analyzer developed by [CoinFabrik's](https://www.coinfabrik.com/) Research and Development team.

We received support through grants from the [Web3 Foundation Grants Program](https://github.com/w3f/Grants-Program/tree/master), the [Aleph Zero Ecosystem Funding Program](https://alephzero.org/ecosystem-funding-program) and the [Stellar Community Fund](https://communityfund.stellar.org).


| Grant Program | Description |
|:-------------:|-------------|
| ![Web3 Foundation](https://raw.githubusercontent.com/CoinFabrik/scout/main/assets/web3-foundation.png) | **Proof of Concept:** We collaborated with the [Laboratory on Foundations and Tools for Software Engineering (LaFHIS)](https://lafhis.dc.uba.ar/) at the [University of Buenos Aires](https://www.uba.ar/internacionales/index.php?lang=en) to establish analysis techniques and tools for our detectors, as well as to create an initial list of vulnerability classes and code examples. [View Grant](https://github.com/CoinFabrik/web3-grant) \| [Application Form](https://github.com/w3f/Grants-Program/blob/master/applications/ScoutCoinFabrik.md).<br><br>**Prototype:** We built a functioning prototype using linting detectors built with [Dylint](https://github.com/trailofbits/dylint) and expanded the list of vulnerability classes, detectors, and test cases. [View Prototype](https://coinfabrik.github.io/scout/) \| [Application Form](https://github.com/w3f/Grants-Program/blob/master/applications/ScoutCoinFabrik_2.md). |
| ![Aleph Zero](https://raw.githubusercontent.com/CoinFabrik/scout/main/assets/aleph-zero.png) | We improved the precision and number of detectors for the tool with a multi-phase approach. This included a manual vulnerability analysis of projects within the Aleph Zero ecosystem, comprehensive testing of the tool on leading projects, and refining its detection accuracy. |
| ![Stellar Community Fund](https://github.com/CoinFabrik/scout-soroban/blob/main/docs/static/img/stellar.png) | We added support for Stellar's smart contract language, Soroban. We included various output formats, such as an HTML report, improved the tool's precision and recall, and added a GitHub action to run the tool with pull requests.|
| ![PAL](https://polkadotassurance.com/wp-content/uploads/2023/03/PAL_logo.svg) | We added support for Substrate pallets in all of Scout's features: CLI, VS Code extension and GitHub Action. |

## About CoinFabrik

We - [CoinFabrik](https://www.coinfabrik.com/) - are a research and development company specialized in Web3, with a strong background in cybersecurity. Founded in 2014, we have worked on over 180 blockchain-related projects, EVM based and also for Solana, Algorand, Stellar and Polkadot. Beyond development, we offer security audits through a dedicated in-house team of senior cybersecurity professionals, currently working on code in Substrate, Solidity, Clarity, Rust, TEAL and Stellar Soroban.

Our team has an academic background in computer science and mathematics, with work experience focused on cybersecurity and software development, including academic publications, patents turned into products, and conference presentations. Furthermore, we have an ongoing collaboration on knowledge transfer and open-source projects with the University of Buenos Aires.

## License

Scout is licensed and distributed under a MIT license. [Contact us](https://www.coinfabrik.com/) if you're looking for an exception to the terms.
