# 🪣 bckt.xyz

Link shortener and secret sharing service.

[![MIT License][license-mit-badge]][license-mit-url]
[![Apache 2.0 License][license-apache-badge]][license-apache-url]
[![Build Status][actions-badge]][actions-url]

[![Buy Me A Coffee][bmac-badge]][bmac-url]
[![GitHub Sponsors][ghs-badge]][ghs-url]

This project is deployed on <https://www.shuttle.rs/> and demonstrates how
one can be built a production-like web service on that platform,
using a codebase written in Rust and make use of dependencies such as
`Tokio`, `Axum`, `Tower`, `Askama`, `Htmx`, `Missing.css` and so on.

> Live @ <https://bckt.xyz>
> 
> (only accessible by friends of plabayo and our projects)



[license-mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-mit-url]: https://github.com/plabayo/bucket/blob/main/LICENSE-MIT
[license-apache-badge]: https://img.shields.io/badge/license-APACHE-blue.svg
[license-apache-url]: https://github.com/plabayo/bucket/blob/main/LICENSE-APACHE
[actions-badge]: https://github.com/plabayo/bucket/workflows/CI/badge.svg
[actions-url]: https://github.com/plabayo/bucket/actions?query=workflow%3ACI+branch%main

[bmac-badge]: https://img.shields.io/badge/Buy%20Me%20a%20Coffee-ffdd00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black
[bmac-url]: https://www.buymeacoffee.com/plabayo
[ghs-badge]: https://img.shields.io/badge/sponsor-30363D?style=for-the-badge&logo=GitHub-Sponsors&logoColor=#EA4AAA
[ghs-url]: https://github.com/sponsors/plabayo

## Project Structure

- Source code of this web service can be found under [`/src`](./src)
  - [`/src/router`](./src/router): logic of the web service endpoints (including root)
  - [`/src/services/auth.rs`](./src/services/auth.rs): authentication of this web service (symmetric encryption, with a magic-link mechanism to login);
- Static assets — found in [`./static`](./static) such as Htmx, a bit of Bckt.xyz logic
   - (e.g. for client-side encryption of secrets),
      and css are served using `tower-http`'s static server using `Axum`;
- Templates are found in [`/templates`](./templates) and are consumed using `Askama`;

In case you have furher questions you can ping `@glendc` at
[Shuttle's Discord](https://discord.gg/YDHm6Yz3).

## Work In Progress

This project is not yet finished. Use at your own risk.

Developer todos:

- create secret logic (dirty)
- allow secrets to be deleted
- import blocklists for all kind of nasty domains which we want to avoid
- add l18n support using `i18n-embed-fl` and `accept-language` crates (for now only english, dutch and spanish support);
- add support for all known languages possible;
- move allowed_email_filters to db storage;
- support invites for users as long as we have less then 50;
- provide API, using same security mechanism
- provide bckt cli tool that over API can communicate with it (config in `~/.bckt.toml`)
- make storage backend swappable with other stuff
- provide also file storage upload using blob storage as backend

## Contributing

🎈 Thanks for your help improving the project! We are so happy to have
you! We have a [contributing guide][contributing] to help you get involved in the
`bucket` project.

Should you want to contribure this project but you do not yet know how to program in Rust, you could start learning Rust with as goal to contribute as soon as possible to `bucket` by using "[the Rust 101 Learning Guide](https://rust-lang.guide/)" as your study companion. Glen can also be hired as a mentor or teacher to give you paid 1-on-1 lessons and other similar consultancy services. You can find his contact details at <https://www.glendc.com/>.

## License

This project is dual-licensed under both the [MIT license][mit-license] and [Apache 2.0 License][apache-license].

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `bucket` by you, shall be licensed as both [MIT][mit-license] and [Apache 2.0][apache-license],
without any additional terms or conditions.

[contributing]: https://github.com/plabayo/bucket/blob/main/CONTRIBUTING.md
[mit-license]: https://github.com/plabayo/bucket/blob/main/LICENSE-MIT
[apache-license]: https://github.com/plabayo/bucket/blob/main/LICENSE-APACHE

## Sponsors

Bucket is **completely free, open-source software** which needs lots of effort and time to develop and maintain.

Support this project by becoming a [sponsor][ghs-url]. One time payments are accepted [at GitHub][ghs-url] as well as at ["Buy me a Coffee"][bmac-url]

Sponsors help us continue to maintain and improve `bucket`, as well as other
Free and Open Source (FOSS) technology. It also helps us to create
educational content such as <https://github.com/plabayo/learn-rust-101>,
and other open source libraries such as <https://github.com/plabayo/tower-async>.

Sponsors receive perks and depending on your regular contribution it also
allows you to rely on us for support and consulting (for any plabayo FOSS project).

### Contribute to Open Source

Part of the money we receive from sponsors is used to contribute to other projects
that we depend upon. Plabayo sponsors the following organisations and individuals
building and maintaining open source software that `bucket` depends upon:

| | name | projects |
| - | - | - |
| 💌 | [Tokio (*)](https://github.com/tokio-rs) | (Tokio Project and Ecosystem)
| 💌 | [Sean McArthur](https://github.com/seanmonstar) | (Hyper and Tokio)
