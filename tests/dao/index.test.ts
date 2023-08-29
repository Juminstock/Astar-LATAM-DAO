import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import DaoFactory from "./typedContract/constructors/dao";
import Dao from "./typedContract/contracts/dao";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";

use(chaiAsPromised);

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944");
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" });

describe("dao test", () => {
  let daoFactory: DaoFactory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  
  let contract: Dao;
  const initialState = true;

  before(async function setup(): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider });
    deployer = keyring.addFromUri("//Alice");

    daoFactory = new DaoFactory(api, deployer);

    contract = new Dao(
      (await daoFactory.new(initialState)).address,
      deployer,
      api
    );
  });

  after(async function tearDown() {
    await api.disconnect();
  });
});
