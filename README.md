# Uniaxe

A small crate to efficiently transform unicode characters into ascii characters, e.g. `
ℍ𝕒𝕙𝕒𝕙 𝕌𝕟𝕚𝕔𝕠𝕕𝕖 𝕝𝕞𝕒𝕠 𝕩𝕕 𝕣𝕠𝕗𝕝` becomes `Hahah Unicode lmao xd rofl`.

## Purpose

Here are some possible uses of this crate:

- Destruction of annoying unicode characters in people's names / messages.
- Destruction of unicode characters in environments where unicode is not allowed.
- Creating a filter for a messaging application (would allow you to stop stupid things like those with a mental age of 2 from spamming `🄵🅄🄲🄺 🅈🄾🅄` and thinking they're the bees knees)

I created this crate for the purpose of my mail spam filter experiment, I wanted to be able to prevent junk mail with unicode characters in them from slipping through and I couldn't find an existing crate or library in any other language.

You may find that this crate can't handle some unicode characters, in that case, you can always submit a pull request or create an issue to allow uniaxe to become more successful at destroying unicode characters.

## Installation
You can install the uniaxe library with ease.

You can add it to your Cargo.toml like so:

```toml
uniaxe = "0"
```

You can also use cargo-edit to add uniaxe to your crate:

```
cargo add uniaxe
```

## Examples
I aim to keep the uniaxe api simple, as it should be.

```rust
use uniaxe::lookup::generate_table;
use uniaxe::uniaxe;

fn main() {
        let table = generate_table();
	let text = uniaxe("𝙡𝙤𝙤𝙠 𝙖𝙩 𝙢𝙚 𝙞'𝙢 𝙨𝙤 𝙦𝙪𝙞𝙧𝙠𝙮", &table);
	println!("{}", text); // This will display "look at me i'm so quirky"
}
```

## Testing
Uniaxe is tested thoroughly to ensure that it will not panic. It also forbids all unsafe code to ensure that software that depends on it is safe.
