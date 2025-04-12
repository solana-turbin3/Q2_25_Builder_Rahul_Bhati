use solana_idlgen::idlgen;

idlgen!(
 {
    "name": "turbine_prereq",
    "version": "0.1.0",
    "metadata": {
        "address": "Trb3aEx85DW1cEEvoqEaBkMn1tfmNEEEPaKzLSu4YAv"
    },
    "instructions": [
        {
            "name": "clean",
            "discriminator": [250, 191, 56, 128, 150, 251, 1, 103],
            "accounts": [
                {
                    "name": "signer",
                    "isMut": false,
                    "isSigner": true
                },
                {
                    "name": "prereq",
                    "isMut": true,
                    "isSigner": false
                }
            ],
            "args": []
        },
        {
            "name": "submit",
            "discriminator": [88, 166, 102, 181, 162, 127, 170, 48],
            "accounts": [
                {
                    "name": "signer",
                    "isMut": true,
                    "isSigner": true
                },
                {
                    "name": "prereq",
                    "isMut": true,
                    "isSigner": false,
                    "pda": {
                        "seeds": [
                            {
                                "kind": "const",
                                "value": [112, 114, 101, 81, 50, 50, 53]
                            },
                            {
                                "kind": "account",
                                "path": "signer"
                            }
                        ]
                    }
                },
                {
                    "name": "system_program",
                    "isMut": false,
                    "isSigner": false,
                    "address": "11111111111111111111111111111111"
                }
            ],
            "args": [
                {
                    "name": "github_username",
                    "type": "bytes"
                }
            ]
        },
        {
            "name": "update",
            "discriminator": [219, 200, 88, 176, 158, 63, 253, 127],
            "accounts": [
                {
                    "name": "signer",
                    "isMut": true,
                    "isSigner": true
                },
                {
                    "name": "prereq",
                    "isMut": true,
                    "isSigner": false
                },
                {
                    "name": "system_program",
                    "isMut": false,
                    "isSigner": false,
                    "address": "11111111111111111111111111111111"
                }
            ],
            "args": [
                {
                    "name": "github",
                    "type": "bytes"
                }
            ]
        },
        {
            "name": "complete",
            "discriminator": [99, 105, 111, 110, 80, 114, 101, 81],
            "accounts": [
                {
                    "name": "signer",
                    "isMut": true,
                    "isSigner": true
                },
                {
                    "name": "prereq",
                    "isMut": true,
                    "isSigner": false
                },
                {
                    "name": "system_program",
                    "isMut": false,
                    "isSigner": false,
                    "address": "11111111111111111111111111111111"
                }
            ],
            "args": [
                {
                    "name": "github",
                    "type": "bytes"
                }
            ]
        }
    ],
    "accounts": [
        {
            "name": "Turbin3PrereqAccount",
            "type": {
                "kind": "struct",
                "fields": [
                    {
                        "name": "github",
                        "type": "bytes"
                    },
                    {
                        "name": "owner",
                        "type": "pubkey"
                    }
                ]
            },
            "discriminator": [210, 203, 168, 103, 251, 233, 204, 6]
        }
    ],
    "errors": [
        {
            "code": 6000,
            "name": "InvalidGithubAccount",
            "msg": "Invalid Github account"
        }
    ],
    "types": [
        {
            "name": "CompleteArgs",
            "type": {
                "kind": "struct",
                "fields": [
                    {
                        "name": "github",
                        "type": "bytes"
                    }
                ]
            }
        }
    ]
}

);
