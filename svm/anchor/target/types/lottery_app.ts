/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/lottery_app.json`.
 */
export type LotteryApp = {
  "address": "C1HzcdTw9f5rEyrEHW6FnmdWo9HuxY4gwW9AJ18EgHJw",
  "metadata": {
    "name": "lotteryApp",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "greet",
      "discriminator": [
        203,
        194,
        3,
        150,
        228,
        58,
        181,
        62
      ],
      "accounts": [],
      "args": []
    }
  ]
};
