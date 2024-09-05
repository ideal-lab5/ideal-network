#!/bin/bash

# Parameters
NODE_URL="http://127.0.0.1:9988"
KEY_TYPE="drnd"
SEED="//Alice"
PUBLIC_KEY="0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"

# Insert the key
curl -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","method":"author_insertKey","params":["'"$KEY_TYPE"'","'"$SEED"'","'"$PUBLIC_KEY"'"],"id":1}' \
     $NODE_URL

# SEED="//Bob"
# PUBLIC_KEY="0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"

# # Insert the key
# curl -H "Content-Type: application/json" \
#      -d '{"jsonrpc":"2.0","method":"author_insertKey","params":["'"$KEY_TYPE"'","'"$SEED"'","'"$PUBLIC_KEY"'"],"id":1}' \
#      $NODE_URL

# SEED="//Charlie"
# PUBLIC_KEY="0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22"

# # Insert the key
# curl -H "Content-Type: application/json" \
#      -d '{"jsonrpc":"2.0","method":"author_insertKey","params":["'"$KEY_TYPE"'","'"$SEED"'","'"$PUBLIC_KEY"'"],"id":1}' \
#      $NODE_URL

# SEED="//Dave"
# PUBLIC_KEY="0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20"

# # Insert the key
# curl -H "Content-Type: application/json" \
#      -d '{"jsonrpc":"2.0","method":"author_insertKey","params":["'"$KEY_TYPE"'","'"$SEED"'","'"$PUBLIC_KEY"'"],"id":1}' \
#      $NODE_URL