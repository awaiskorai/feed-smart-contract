import type { Env } from "@terra-money/terrain";
import { FeedContractClient } from './clients/FeedContractClient';

export class Lib extends FeedContractClient {
  env: Env;

  constructor(env: Env) {
    super(env.client, env.defaultWallet, env.refs['feed-contract'].contractAddresses.default);
    this.env = env;
  }
};

export default Lib;
