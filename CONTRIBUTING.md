## Contributing

Changes **should not be**:
* committed to branch main/master
* committed to branch testing

Changes **should be**:
* committed to development
* *merged* with testing and pushed

Changes merged into testing will be automatically built with the latest stable rustc  
 on the following github-hosted docker images (see [github docs][1]):

* ubuntu-18.04
* ubuntu-20.04
* windows-latest
* macos-latest

[1]: <https://docs.github.com/en/actions/reference/specifications-for-github-hosted-runners#supported-runners-and-hardware-resources> (Supported runners and hardware resources)