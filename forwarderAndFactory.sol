
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "@openzeppelin/contracts/proxy/Clones.sol";

interface IERC20 {
    function transfer(address to, uint256 value) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}


contract Forwarder {

    address public owner;

    constructor(address _owner) public {
            owner = _owner;
    }


    modifier onlyOwner {
        require(msg.sender == owner);
        _;
    }

    function setOwner(address _owner)public onlyOwner returns (bool){
      owner = _owner;
      return true;
    }

    function flushERC20(address tokenContractAddress, address destination)external onlyOwner returns (uint256){
      IERC20 tokenContract = IERC20(tokenContractAddress);
      uint256 forwarderBalance = tokenContract.balanceOf(address(this));
      tokenContract.transfer(destination, forwarderBalance);
      return forwarderBalance;

    }

    function flushGivenERC20Amount(uint256 amount, address tokenContractAddress, address destination)external onlyOwner returns (uint256){
      IERC20 tokenContract = IERC20(tokenContractAddress);
      uint256 forwarderBalance = tokenContract.balanceOf(address(this));
      if(amount>=forwarderBalance){
        tokenContract.transfer(destination, forwarderBalance);
        return forwarderBalance;
      }else{
        tokenContract.transfer(destination, amount);
        return amount;
      }
    }

    function init(address _owner) public {
      require(owner == address(0x0));
      owner = _owner;
    }



}

contract ForwarderFactory {
  event Deployed(address indexed addr, address indexed owner);

  // uint256 saltIndex = 30;
  address public owner;
  uint256 public amountTemp;
  address[] public accountList;

  constructor() public {
          owner = msg.sender;
  }
  modifier onlyOwner {
      require(msg.sender == owner);
      _;
  }

  function addAccounts(address tempAccount) public{
   
    accountList.push(tempAccount);

  }

  function flushSmartAccountERC20(address forwarder, address tokenAddress, address _to) public  returns(uint256){
        (bool success,bytes memory responseData) = forwarder.call(abi.encodeWithSignature("flushERC20(address,address)", tokenAddress, _to));
        require(success, "flushERC20 failed");
       return abi.decode(responseData, (uint256));     
  }

  function flushSmartAccountWithGivenERC20Amount( address forwarder, uint256 amount, address tokenAddress, address _to) public  returns(uint256){

        (bool success ,bytes memory responseData) = forwarder.call(abi.encodeWithSignature("flushGivenERC20Amount(uint256,address,address)", amount, tokenAddress, _to));
        require(success, "flushGivenERC20Amount failed");
        return abi.decode(responseData, (uint256));   
  }

  function flushAccountsERC20(uint256 amount, address tokenAddress, address destination) public {
    IERC20 tokenContract = IERC20(tokenAddress);
    require(accountList.length > 0, "account list empty");
    for(uint i=0; i<accountList.length; i++){

      if(amount > 0 ){

        if( accountList[i] != destination && Forwarder(accountList[i]).owner() == address(this) && tokenContract.balanceOf(accountList[i]) > 0){

          uint256 transferedAmount = flushSmartAccountWithGivenERC20Amount( accountList[i], amount, tokenAddress, destination);
          amount = amount - transferedAmount;
        }
      }else{

        break;
      }


    }

    require(amount == 0, "accounts not have enough balance");

  }

  function changeSmartAccountOwner(address forwarder, address ownerAddress) public {
      (bool success,) = forwarder.call(abi.encodeWithSignature("setOwner(address)", ownerAddress));
      require(success, "setOwner failed");
  }

  function createSmartAccountClones(address forwarder, uint256 walletCount, uint256 saltIndex) public returns (address[] memory walletAddressList) {

    walletAddressList = new address[](walletCount);

    for(uint i=0; i < walletCount; i++){
      // cloneForwarder(forwarder, saltIndex);

      address clonedAddress = createClone(forwarder, saltIndex);
      Forwarder clonedForwarder = Forwarder(clonedAddress);
      clonedForwarder.init(address(this));
      walletAddressList[i] = clonedAddress;
      accountList.push(clonedAddress);
      emit Deployed(clonedAddress, msg.sender);
      saltIndex++;

    }

  }

  function cloneForwarder(address forwarder, uint256 salt)
      public returns (Forwarder clonedForwarder) {

    // address clonedAddress = createClone(forwarder, salt);
    // Forwarder parentForwarder = Forwarder(forwarder);
    // clonedForwarder = Forwarder(clonedAddress);

    // clonedForwarder.init(parentForwarder.destination());
    // address clonedAddress = createClone(forwarder, salt);
    // clonedForwarder = Forwarder(clonedAddress);

    address clonedAddress = createClone(forwarder, salt);
    clonedForwarder = Forwarder(clonedAddress);
    clonedForwarder.init(address(this));
    accountList.push(clonedAddress);
    emit Deployed(clonedAddress, msg.sender);
  }

  function createClone(address target, uint256 salt) private returns (address result) {
    bytes20 targetBytes = bytes20(target);
    assembly {
      let clone := mload(0x40)
      mstore(clone, 0x3d602d80600a3d3981f3363d3d373d3d3d363d73000000000000000000000000)
      mstore(add(clone, 0x14), targetBytes)
      mstore(add(clone, 0x28), 0x5af43d82803e903d91602b57fd5bf30000000000000000000000000000000000)
      result := create2(0, clone, 0x37, salt)
    }
  }

  function flushERC20(address tokenContractAddress, address destination)external onlyOwner returns (bool){
    IERC20 tokenContract = IERC20(tokenContractAddress);
    uint256 forwarderBalance = tokenContract.balanceOf(address(this));
    tokenContract.transfer(destination, forwarderBalance);
    return true;

  }
  

  function predictCloneAddress(address forwarderAddress_, uint256 salt_)
    public
    view
    returns (address)
        {
            address predictedAddress =
                Clones.predictDeterministicAddress(
                    forwarderAddress_,
                    bytes32(salt_)
                );
    
            return predictedAddress;
        }

}
