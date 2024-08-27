import json
from web3 import Web3
from hexbytes import HexBytes

class HexJsonEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, HexBytes):
            return obj.hex()
        return super().default(obj)

def read_inputs():
    f = open('config.json')
    data = json.load(f)
    return data

inputs = read_inputs()

# x=get_trader_nickName("C30299E36462A99EA5A0DE7F03C8F9DD"):
mainnet_provider_url = inputs['mainnet_provider_url']
testnet_provider_url = inputs['testnet_provider_url']
chain_ID = int(inputs['chain_ID'])

account_address = inputs['account_address']
account_private_key = inputs['account_private_key']

tx_gas_price = inputs['tx_gas_price']
tx_gas_limit = inputs['tx_gas_limit']

transfer_token_amount = inputs['transfer_token_amount']
token_address = inputs['token_address']
destination_address = inputs['destination_address']

forwarder_address = inputs['forwarder_address']
create_accounts_count = inputs['create_accounts_count']
salt = inputs['salt']
forwarder_factory_address = inputs['forwarder_factory_address']
forwarder_factory_ABI = inputs['forwarder_factory_ABI']

if chain_ID == 1:
    provider = Web3.HTTPProvider(mainnet_provider_url)
else:
    provider = Web3.HTTPProvider(testnet_provider_url)

# web3 = Web3(Web3.HTTPProvider(bsc))
print(str(provider))
w3 = Web3(provider)
print(str(w3))

def generate_forwarder_clones():
    print("Going to create "+str(create_accounts_count)+" accounts")
    factory =  w3.eth.contract( address=forwarder_factory_address, abi=forwarder_factory_ABI)
    # ethTX = factory.methods.createSmartAccountClones( forwarder_address, create_accounts_count, salt )
    ethTXData = factory.encodeABI(fn_name='createSmartAccountClones', args=[ forwarder_address, create_accounts_count, salt ])
    ethTXParam = {
        "from" : account_address,
        "to": forwarder_factory_address,
        "nonce": w3.eth.get_transaction_count(account_address),
        "gasPrice": w3.toWei('13', 'gwei'), 
        "gas": tx_gas_limit,
        "data": ethTXData,
        "chainId": chain_ID
    }
    signed_txn = w3.eth.account.sign_transaction(ethTXParam, account_private_key)
    txn_hash = w3.eth.send_raw_transaction(signed_txn.rawTransaction)
    print("Tx hash : "+w3.toHex(txn_hash))
    print("Tx mining... ")
    receipt = w3.eth.wait_for_transaction_receipt(txn_hash)
    # for i in range(len(vars(receipt)['logs'])):
    xx = vars(receipt)['logs']
    print("============================")
    for x in xx:
        # print(vars(x))
        print("============================")
        # print(vars(x)['topics'])
        xxx = vars(x)['topics']
        addressCount = 0
        for y in xxx:
            temp = w3.toHex(y)

            temp = '0x' + temp[-40:]
            if addressCount == 1:
                print("Account address : "+temp)
            addressCount = addressCount + 1

def flush():
    print("flush()")
    factory =  w3.eth.contract( address=forwarder_factory_address, abi=forwarder_factory_ABI)
    # ethTX = factory.methods.createSmartAccountClones( forwarder_address, create_accounts_count, salt )
    ethTXData = factory.encodeABI(fn_name='flushAccountsERC20', args=[ transfer_token_amount, token_address, destination_address ])
    ethTXParam = {
        "from" : account_address,
        "to": forwarder_factory_address,
        "nonce": w3.eth.get_transaction_count(account_address),
        "gasPrice": tx_gas_price, 
        "gas": tx_gas_limit,
        "data": ethTXData,
        "chainId": chain_ID
    }
    signed_txn = w3.eth.account.sign_transaction(ethTXParam, account_private_key)
    txn_hash = w3.eth.send_raw_transaction(signed_txn.rawTransaction)
    print("Tx hash : "+w3.toHex(txn_hash))
    print("Tx mining... ")
    receipt = w3.eth.wait_for_transaction_receipt(txn_hash)
    print(vars(receipt))


"""CODE BEGINS HERE"""
val = input("Enter 1 to create addresses. \nEnter 2 to flush. \n")
if val == "1":
    generate_forwarder_clones()
elif val == "2":
    flush()
else:
    print("wrong input")


    