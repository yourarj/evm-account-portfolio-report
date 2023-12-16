use ethers::contract::abigen;

abigen!(
  IERC20,
  r#"[
    event Approval(address indexed owner, address indexed spender, uint256 value)
    event Transfer(address indexed from, address indexed to, uint256 value)
    function decimals() external view returns (uint8)
    function totalSupply() external view returns (uint256)
    function balanceOf(address owner) external view returns (uint256)
    function allowance(address owner, address spender) external view returns (uint256)
    function approve(address spender, uint256 value) external returns (bool)
    function transfer(address to, uint256 value) external returns (bool)
    function transferFrom(address from, address to, uint256 value) external returns (bool)
    ]"#,
);
