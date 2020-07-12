# CloudFlare worker speed test

See https://community.cloudflare.com/t/cloudflare-workers-very-slow-with-moderate-sized-webassembly-bindings/184668/12

This worker does one very simple thing it substitutes the variable `{{ name }}` in a string and returns that string.

By default the template is `Hello, {{ name }}` and the value of name is `world`, both `template` and `name` can
be customised using get parameters.

The substitution can be done in the the following ways:
* in javascript using `replace`
* in rust use string `replace`
* in rust using [regex](https://crates.io/crates/regex) `replace_all`
* use a tera [tera](https://crates.io/crates/tera) `one_off`

Using the following means of substitution varies the was module from non existing, from non existent, to small, 
medium and large.

## Performance results

Results are calculated using `test_performance.py` script above.

...
