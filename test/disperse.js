const { default: axios } = require("axios");

const fixed_amount_data = {
  amountToDistribute: "159355000000000",
  senderPrivateKey: "SENDER_PRIVATE_KEY_2", // env
  receivers: ["0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326", "0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5"],
  shares: ["79677500000000", "79677500000000"],
  fixedType: true
}

const percentage_data = {
  amountToDistribute: "159355000000000000",
  senderPrivateKey: "SENDER_PRIVATE_KEY", // env
  receivers: ["0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326", "0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5"],
  shares: ["85", "15"],
  fixedType: false
}

const fixed_amount_erc20_data = {
  token: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
  amountToDistribute: "15000000000",
  senderPrivateKey: "SENDER_PRIVATE_KEY", // env
  receivers: ["0x5B1D72Dce914FC4fB24d2BfBa4DdBdd05625152D"],
  shares: ["15000000000"],
  fixedType: true
}

const percentage_erc20_data = {
  token: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
  amountToDistribute: "15935500000",
  senderPrivateKey: "SENDER_PRIVATE_KEY", // env
  receivers: ["0x5B1D72Dce914FC4fB24d2BfBa4DdBdd05625152D", "0x5B1D72Dce914FC4fB24d2BfBa4DdBdd05625152D"],
  shares: ["74", "26"],
  fixedType: false
}

async function main() {
  let data;

  switch (process.argv[2]) {
    case "1":
      data = fixed_amount_data;
      break;
    case "2":
      data = percentage_data;
      break;
    case "3":
      data = fixed_amount_erc20_data;
      break;
    case "4":
      data = percentage_erc20_data;
      break;
    default:
      console.log("Invalid command");
      process.exit(1);
  }

  let resp;
  try {
    resp = await axios.post(
      `http://127.0.0.1:7777/api/disperse`,
      data
    );
    console.log(resp.data);
  } catch (error) {
    console.log(error);
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
