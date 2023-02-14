### OPCODE COVERAGE

> _Yes, the tables are stolen from [vEVM's README](https://github.com/kethcode/vEVM/blob/main/README.md) because I'm a lazy ass._

No. of OPCODEs implemented : **38/141**

### 0x00 range - Stop and Arithmetic Operations

| Mnemonic   | OpCode | Status |
| ---------- | ------ | ------ |
| STOP       | 0x00   | Done   |
| ADD        | 0x01   | Done   |
| MUL        | 0x02   | Done   |
| SUB        | 0x03   | Done   |
| DIV        | 0x04   | Done   |
| SDIV       | 0x05   | Done   |
| MOD        | 0x06   | Done   |
| SMOD       | 0x07   | Done   |
| ADDMOD     | 0x08   | Done   |
| MULMOD     | 0x09   | Done   |
| EXP        | 0x0A   | Done   |
| SIGNEXTEND | 0x0B   |        |

### 0x10 range - Comparison & Bitwise Logic Operations

| Mnemonic | OpCode | Status |
| -------- | ------ | ------ |
| LT       | 0x10   | Done   |
| GT       | 0x11   | Done   |
| SLT      | 0x12   | Done   |
| SGT      | 0x13   | Done   |
| EQ       | 0x14   | Done   |
| ISZERO   | 0x15   | Done   |
| AND      | 0x16   | Done   |
| OR       | 0x17   | Done   |
| XOR      | 0x18   | Done   |
| NOT      | 0x19   | Done   |
| BYTE     | 0x1A   | Done   |
| SHL      | 0x1B   | Done   |
| SHR      | 0x1C   | Done   |
| SAR      | 0x1D   |        |

### 0x20 range - SHA3

| Mnemonic | OpCode | Status |
| -------- | ------ | ------ |
| SHA3     | 0x20   |        |

### 0x30 range - Environmental **Information**

| Mnemonic       | OpCode | Status |
| -------------- | ------ | ------ |
| ADDRESS        | 0x30   | Done   |
| BALANCE        | 0x31   | Done   |
| ORIGIN         | 0x32   |        |
| CALLER         | 0x33   | Done   |
| CALLVALUE      | 0x34   | Done   |
| CALLDATALOAD   | 0x35   |        |
| CALLDATASIZE   | 0x36   |        |
| CALLDATACOPY   | 0x37   |        |
| CODESIZE       | 0x38   |        |
| CODECOPY       | 0x39   |        |
| GASPRICE       | 0x3A   |        |
| EXTCODESIZE    | 0x3B   |        |
| EXTCODECOPY    | 0x3C   |        |
| RETURNDATASIZE | 0x3D   |        |
| RETURNDATACOPY | 0x3E   |        |
| EXTCODEHASH    | 0x3F   |        |

### 0x40 range - Block Information

| Mnemonic    | OpCode | Status |
| ----------- | ------ | ------ |
| BLOCKHASH   | 0x40   |        |
| COINBASE    | 0x41   |        |
| TIMESTAMP   | 0x42   |        |
| NUMBER      | 0x43   |        |
| PREVRANDAO  | 0x44   |        |
| GASLIMIT    | 0x45   |        |
| CHAINID     | 0x46   |        |
| SELFBALANCE | 0x47   | Done   |
| BASEFEE     | 0x48   |        |

### 0x50 range - Stack Memory Storage and Flow Operations

| Mnemonic | OpCode | Status |
| -------- | ------ | ------ |
| POP      | 0x50   | Done   |
| MLOAD    | 0x51   | Done   |
| MSTORE   | 0x52   | Done   |
| MSTORE8  | 0x53   | Done   |
| SLOAD    | 0x54   |        |
| SSTORE   | 0x55   |        |
| JUMP     | 0x56   | Done   |
| JUMPI    | 0x57   | Done   |
| PC       | 0x58   | Done   |
| MSIZE    | 0x59   | Done   |
| GAS      | 0x5A   |        |
| JUMPDEST | 0x5B   | Done   |

### 0x60 range - Push Operations

| Mnemonic | OpCode | Status |
| -------- | ------ | ------ |
| PUSH1    | 0x60   | Done   |
| PUSH2    | 0x61   | Done   |
| PUSH3    | 0x62   | Done   |
| PUSH4    | 0x63   |        |
| PUSH5    | 0x64   |        |
| PUSH6    | 0x65   |        |
| PUSH7    | 0x66   |        |
| PUSH8    | 0x67   |        |
| PUSH9    | 0x68   |        |
| PUSH10   | 0x69   |        |
| PUSH11   | 0x6A   |        |
| PUSH12   | 0x6B   |        |
| PUSH13   | 0x6C   |        |
| PUSH14   | 0x6D   |        |
| PUSH15   | 0x6E   |        |
| PUSH16   | 0x6F   |        |
| PUSH17   | 0x70   |        |
| PUSH18   | 0x71   |        |
| PUSH19   | 0x72   |        |
| PUSH20   | 0x73   |        |
| PUSH21   | 0x74   |        |
| PUSH22   | 0x75   |        |
| PUSH23   | 0x76   |        |
| PUSH24   | 0x77   |        |
| PUSH25   | 0x78   |        |
| PUSH26   | 0x79   |        |
| PUSH27   | 0x7A   |        |
| PUSH28   | 0x7B   |        |
| PUSH29   | 0x7C   |        |
| PUSH30   | 0x7D   |        |
| PUSH31   | 0x7E   |        |
| PUSH32   | 0x7F   |        |

### 0x80 range - Duplication Operations

| Mnemonic | OpCode | Status |
| -------- | ------ | ------ |
| DUP1     | 0x80   | Done   |
| DUP2     | 0x81   |        |
| DUP3     | 0x82   |        |
| DUP4     | 0x83   |        |
| DUP5     | 0x84   |        |
| DUP6     | 0x85   |        |
| DUP7     | 0x86   |        |
| DUP8     | 0x87   |        |
| DUP9     | 0x88   |        |
| DUP10    | 0x89   |        |
| DUP11    | 0x8A   |        |
| DUP12    | 0x8B   |        |
| DUP13    | 0x8C   |        |
| DUP14    | 0x8D   |        |
| DUP15    | 0x8E   |        |
| DUP16    | 0x8F   |        |

### 0x90 range - Exchange Operations

| Mnemonic | OpCode | Status |
| -------- | ------ | ------ |
| SWAP1    | 0x90   | Done   |
| SWAP2    | 0x91   |        |
| SWAP3    | 0x92   |        |
| SWAP4    | 0x93   |        |
| SWAP5    | 0x94   |        |
| SWAP6    | 0x95   |        |
| SWAP7    | 0x96   |        |
| SWAP8    | 0x97   |        |
| SWAP9    | 0x98   |        |
| SWAP10   | 0x99   |        |
| SWAP11   | 0x9A   |        |
| SWAP12   | 0x9B   |        |
| SWAP13   | 0x9C   |        |
| SWAP14   | 0x9D   |        |
| SWAP15   | 0x9E   |        |
| SWAP16   | 0x9F   |        |

### 0xa0 range - logging ops

| Mnemonic | OpCode | Status |
| -------- | ------ | ------ |
| LOG0     | 0xA0   |        |
| LOG1     | 0xA1   | Done   |
| LOG2     | 0xA2   |        |
| LOG3     | 0xA3   |        |
| LOG4     | 0xA4   |        |

### 0xb0 range

| Mnemonic | OpCode | Status |
| -------- | ------ | ------ |
| TLOAD    | 0xB3   |        |
| TSTORE   | 0xB4   |        |

### 0xf0 range - closures

| Mnemonic     | OpCode | Status |
| ------------ | ------ | ------ |
| CREATE       | 0xF0   |        |
| CALL         | 0xF1   |        |
| CALLCODE     | 0xF2   |        |
| RETURN       | 0xF3   | Done   |
| DELEGATECALL | 0xF4   |        |
| CREATE2      | 0xF5   |        |
| STATICCALL   | 0xFA   |        |
| REVERT       | 0xFD   |        |
| INVALID      | 0xFE   | Done   |
| SELFDESTRUCT | 0xFF   |        |
