#!/bin/bash

# Parameters
NODE_URL="http://127.0.0.1:$1"
KEY_TYPE="drnd"
SEED="//Alice"
PUBLIC_KEY="0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"

# Insert the key
curl -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","method":"author_insertKey","params":["'"$KEY_TYPE"'","'"$SEED"'","'"$PUBLIC_KEY"'"],"id":1}' \
     $NODE_URL
