// Use Jest to test

import { idlFactory as exampleIDL } from '@/idls/ego_example.idl';
import { _SERVICE as exampleService } from '@/idls/ego_example';
import { getCanisterId, getActor, identity } from '@ego-js/utils';
import { ActorSubclass } from '@dfinity/agent';

describe('ego_example', () => {
  let exampleActor: ActorSubclass<exampleService>;
  test('who am i', async () => {
    exampleActor =
      // getActor use idl types
      await getActor<exampleService>(
        // use credential identity, owner of canister
        identity(),
        // use idlFactory from generated file
        exampleIDL,
        // get canister ID for 'ego_example', `configs/ego_example.json` is generated
        getCanisterId('ego_example')!,
      );
    const pid = (await exampleActor.whoAmI()).toText();

    expect(pid).toBe(identity().getPrincipal().toText());
  });
  test('insert_btree', async () => {
    await exampleActor.insert_btree('hello', { key: 'world_bytes', value: Array.from([]) });
    await exampleActor.insert_btree('hello2', { key: 'world_bytes2', value: Array.from([]) });
    const result = await exampleActor.get_btree('hello');
    expect(result).toEqual([{ key: 'world_bytes', value: Uint8Array.from([]) }]);
    const result_2 = await exampleActor.get_all_btree();
    expect(result_2).toEqual([
      { key: 'world_bytes', value: Uint8Array.from([]) },
      { key: 'world_bytes2', value: Uint8Array.from([]) },
    ]);
  });
});
