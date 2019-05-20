# do-kube-config-updater

A naïve tool to create or update your local kubernetes config with a config from a DigitalOcean managed kubernetes cluster.

> NOTE: This is essentially just tested for me and my machine. Your mileage may vary!

# Installing

You have two options to get this tool on your machine.

1. Download a version from the releases page and put it somewhere in your `PATH`
2. Download the source files, compile, and run the code yourself. See the [contributing](#contributing) section below

# Usage

As of right now, this cli tool doesn't take any parameters. There are two environment variables that are required for this to work

- `CLUSTER_ID`: The ID of your k8s cluster on DigitalOcean
- `DO_API_KEY`: An API key for your DigitalOcean account

Once those are in place, you can run the binary by itself.

# Contributing

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.

I would love to see issues and pull requests to make this a better tool that works for people other than myself!

This project only works with rust's 2018 edition. Thus, you must have version 1.31 or later. Once you have rust installed, you can then run `cargo run` to see it in action. This will download and compile all the dependencies in development mode.

# [License](LICENSE.md)
