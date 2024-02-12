# mask-cli
[![Build](https://github.com/infrastructure-blocks/mask-cli/actions/workflows/build.yml/badge.svg)](https://github.com/infrastructure-blocks/mask-cli/actions/workflows/build.yml)
[![Update From Template](https://github.com/infrastructure-blocks/mask-cli/actions/workflows/update-from-template.yml/badge.svg)](https://github.com/infrastructure-blocks/mask-cli/actions/workflows/update-from-template.yml)
[![codecov](https://codecov.io/gh/infrastructure-blocks/mask-cli/graph/badge.svg?token=91PJG61LWD)](https://codecov.io/gh/infrastructure-blocks/mask-cli)

This command line utility is used to wrap the invocation of another command and mask its output given the provided
configuration.

Which tokens to be masked from the invocation's output can be provided in one of several ways:
- Through `stdin`
- As environment variables
- Using command line options

See the [usage](#usage) section for more info.

## Usage

We don't recommend passing the tokens directly on the CLI to hide it from the shell history. However, for simplicity's
sake, we may present examples using this paradigm.

### As an environment variable

```shell
#!/usr/bin/env bash

# The default tokens separator expression is any whitespace character.
export MASK_TOKENS="token-1 token-2 token-3"

# When my-app outputs one of ["token-1", "token-2", "token-3"], it is masked and replaced by a "*" padded string of
# equal length. "*******" in the example provided. 
mask my-app
```
