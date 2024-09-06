const { default: axios } = require("axios");

const collect_fixed_eth_data = {
  ownerPrivateKey: "OWNER_PRIVATE_KEY", // env
  sendersWithShares: [
    {
      privateKey: "SENDER_PRIVATE_KEY_2",
      share: "75000000000000000",
    },
    {
      privateKey: "SENDER_PRIVATE_KEY_2",
      share: "75000000000000000",
    },
  ],
  receiver: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
  withdrawAmount: "150000000000000000",
  fixedType: true,
};

const collect_percentage_eth_data = {
  ownerPrivateKey: "OWNER_PRIVATE_KEY", // env
  sendersWithShares: [
    {
      privateKey: "SENDER_PRIVATE_KEY_2",
      share: "7500000000000000000",
    },
    {
      privateKey: "SENDER_PRIVATE_KEY_2",
      share: "7500000000000000000",
    },
  ],
  receiver: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
  withdrawAmount: "100",
  fixedType: false,
};

const collect_fixed_erc20_data = {
  token: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
  ownerPrivateKey: "OWNER_PRIVATE_KEY", // env
  sendersWithShares: [
    {
      privateKey: "SENDER_PRIVATE_KEY_2",
      share: "7500000000",
    },
    {
      privateKey: "SENDER_PRIVATE_KEY_2",
      share: "7500000000",
    },
  ],
  receiver: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
  withdrawAmount: "15000000000",
  fixedType: true,
};

const collect_percentage_erc20_data = {
  token: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
  ownerPrivateKey: "OWNER_PRIVATE_KEY", // env
  sendersWithShares: [
    {
      privateKey: "SENDER_PRIVATE_KEY_2",
      share: "7500000000",
    },
    {
      privateKey: "SENDER_PRIVATE_KEY_2",
      share: "7500000000",
    },
  ],
  receiver: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
  withdrawAmount: "90",
  fixedType: false,
};

async function main() {
  let data;

  switch (process.argv[2]) {
    case "1":
      data = collect_fixed_eth_data;
      break;
    case "2":
      data = collect_percentage_eth_data;
      break;
    case "3":
      data = collect_fixed_erc20_data;
      break;
    case "4":
      data = collect_percentage_erc20_data;
      break;
    default:
      console.log("Invalid command");
      process.exit(1);
  }

  let resp;
  try {
    resp = await axios.post(`http://127.0.0.1:7777/api/collect`, data);
    console.log(resp.data);
  } catch (error) {
    console.log(error);
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
