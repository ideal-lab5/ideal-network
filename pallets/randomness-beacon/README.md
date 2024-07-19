# pallet-randomness-beacon

This pallet bridges to a randomness beacon via a relayer. It constructs a **forkless** chain of verifiable randomness following NIST's proposed randomness beacon specification.

## How it works

The pallet starts at a genesis block (not necessarily network genesis). 
An untrusted relayer component interpolates signatures and pushes them to the beacon.
The beacon verifies the signature and encodes it into storage.
Assume it is using Sha512.
It does this in a way that builds a hash-chain, where each entry looks like:

https://nvlpubs.nist.gov/nistpubs/ir/2019/NIST.IR.8213-draft.pdf

```json
{
    "header": {
        "block_number": number,
        "hash(prev_sig)": string,
        "metadata": "todo",
    },
    "body": {
        "sig": string,
        "proof": string
    }
}
```

Where the metadata field is defined following the randomness beacon standard proposed by NIST. Thus, the metadata contains the following 21 fields:

NISTIR 8213 (DRAFT) A REFERENCE FOR RANDOMNESS BEACONS
``` json
[
    {
        "name": "uri",
        "description": "a uniform resource identifier (URI) that uniquely identifies the pulse",
        "type": "string"
    },
    {
        "name": "version",
        "description": "the version of the beacon format being used",
        "type": "string"
    },
    {
        "name": "cipherSuite",
        "description": "the ciphersuite (set of cryptographic algorithms) being used",
        "type": "string"
    },
    {
        "name": "period",
        "description": "the number (denoted by π) of milliseconds between the timestamps of this pulse and the expected subsequent pulse",
        "type": "integer"
    },
    {
        "name": "certificateId",
        "description": "the hash of the certificate that allows verifying the signature in the pulse; the full certificate must be available via the website of the beacon",
        "type": "string"
    },
    {
        "name": "chainIndex",
        "description": "the chain index (integer identifier, starting at 1) of the chain to which the pulse belongs",
        "type": "integer"
    },
    {
        "name": "pulseIndex",
        "description": "the pulse index (integer identifier, starting at 1), conveying the order of generation of this pulse within its chain",
        "type": "integer"
    },
    {
        "name": "timeStamp",
        "description": "the time (UTC) of pulse release by the Beacon Engine (the actual release time MAY be slightly larger, but SHALL NOT be smaller)",
        "type": "string"
    },
    {
        "name": "localRandomValue",
        "description": "the hash() of two or more high-quality random bit sources",
        "type": "string"
    },
    {
        "name": "external.sourceId",
        "description": "the hash() of a text description of the source of the external value, or an all-zeros byte string if there is no external value",
        "type": "string"
    },
    {
        "name": "external.statusCode",
        "description": "the status of the external value",
        "type": "string"
    },
    {
        "name": "external.value",
        "description": "the hash() of an external value, drawn from a verifiable external source from time to time, or an all-zeros string if there is no external value",
        "type": "string"
    },
    {
        "name": "previous",
        "description": "the outputValue of the previous pulse",
        "type": "string"
    },
    {
        "name": "hour",
        "description": "the outputValue of the first pulse in the (UTC) hour of the previous pulse",
        "type": "string"
    },
    {
        "name": "day",
        "description": "the outputValue of the first pulse in the (UTC) day of the previous pulse",
        "type": "string"
    },
    {
        "name": "month",
        "description": "the outputValue of the first pulse in the (UTC) month of the previous pulse",
        "type": "string"
    },
    {
        "name": "year",
        "description": "the outputValue of the first pulse in the (UTC) year of the previous pulse",
        "type": "string"
    },
    {
        "name": "precommitmentValue",
        "description": "the hash() of the next pulse’s localRandomValue",
        "type": "string"
    },
    {
        "name": "statusCode",
        "description": "the status of the chain at this pulse",
        "type": "string"
    },
    {
        "name": "signatureValue",
        "description": "a signature on all the above fields",
        "type": "string"
    },
    {
        "name": "outputValue",
        "description": "the hash() of all the above fields",
        "type": "string"
    }
]

```