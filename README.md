# gm_brotli

[brotli](https://crates.io/crates/brotli) bindings for Garry's Mod, published only for archiving purposes (written for my specific tasks) so don't expect any support.

## why

Brotli seems to compress data (lua files specifically) better than LZMA used in [util.Compress](https://gmodwiki.com/util.Compress) while also allowing us to choose level of data compression.

| Original | 7zip.7zip (LZMA)      | Google.Brotli      | Meta.ZStandard     |
| -------- | --------------------- | ------------------ | ------------------ |
| 17.4 KiB | 4.09 KiB (-76.49%)    | 3.55 KiB (-79.60%) | 4.06 KiB (-76.67%) |

## api

```lua
local data = 'blahblah lahlalhalha'

local compressed = brotli.compress(data, 11) --[[ 11 is max compression ]]
local decompressed = brotli.decompress(compressed)

assert(decompressed == data, 'decompressed is not same as starting data')
```