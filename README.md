> [!IMPORTANT]
> This tool is **for educational purposes only**, I am not responsible for its use and it was not intended to cause harm.

# 👾 **R**usty **R**aid **B**ot

Rusty Raid Bot (RRB) is a Discord raid tool written in Rust, designed to be used through a minimalist command line interface. It focuses on **performance** and **ease of use** to execute tasks with hundreds or thousands of requests almost instantly.

## Features

- ⚡ **Extreme Speed:** Create/Delete Channels and Roles, Send Messages, Ban/Kick Users in Bulk, and more in just a few milliseconds.
- 🎨 **Customization:** Customize spam messages, channel/role names and descriptions, and more to your liking.
- 📃 **Logging:** Includes a log section that documents all executed actions and catches potential errors.
- 👀 **Simple Interface:** Everything is handled through an easy-to-use terminal interface.
- 🚀 **General purpose:** Provide your bot token and get a list of available servers to perform tasks.

## Rate limit

RRB makes hundreds, or even thousands, of requests in parallel, which can lead to failures due to API rate limits. This can be handled internally by providing an **IP or list of IPs** to use as **proxies**, which can help avoid overall tool blocking. A section for this is provided in the same interface.

## License

This project is licensed under the [MIT License](./LICENSE).