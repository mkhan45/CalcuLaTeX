## Contributing

All changed code should be: 

* Applied to the development branch (pull request/commit etc)
* Manually merged with testing and pushed

**Direct commits to testing or main are discouraged, all changes should be merged into testing from development.**

Changes merged into testing will be automatically built on the following github-hosted docker images (see [github docs][1]):

* ubuntu-18.04
* ubuntu-20.04
* windows-latest

[1]: <https://docs.github.com/en/actions/reference/specifications-for-github-hosted-runners#supported-runners-and-hardware-resources> (Supported runners and hardware resources)