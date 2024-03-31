# tprelay

[![Deploy with Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/skyf0cker/tprelay/tree/main/templates/experimental/tprelay)


tprelay provides a api for your telegraph posts on cloudflare workers.

## Setup
```shell
# set your telegraph access token
# or: npx wrangler secret put TELEGRAPH_ACCESS_TOKEN
wrangler secret put TELEGRAPH_ACCESS_TOKEN
```

## Example

```shell
# page_num: page number, start from 0 [optional]
# page_size: page size, default size is 10 [optional]
curl -XGET http://your-endpoint\?page_num\=0\&page_size\=10
```