import { useState } from "react";
import axios from "axios";
import {
  Button,
  HStack,
  Link,
  Menu,
  MenuButton,
  MenuDivider,
  MenuItem,
  MenuList,
  Text,
  VStack,
} from "@chakra-ui/react";
import {
  SubstrateChain,
  SubstrateWallet,
  SubstrateWalletPlatform,
  allSubstrateWallets,
  getSubstrateChain,
  isWalletInstalled,
  useBalance,
  useInkathon,
} from "@scio-labs/use-inkathon";
import { AiOutlineCheckCircle, AiOutlineDisconnect } from "react-icons/ai";
import { FiChevronDown, FiExternalLink } from "react-icons/fi";
import { SignerPayloadRaw } from "@polkadot/types/types";

export default function ConnectWalletButton({
  setIsSigning,
}: {
  setIsSigning: Function;
}) {
  const {
    activeChain,
    switchActiveChain,
    connect,
    disconnect,
    isConnecting,
    activeAccount,
    accounts,
    setActiveAccount,
    activeSigner,
  } = useInkathon();

  const [hasSigned, setHasSigned] = useState(false);
  const [isSuccessfulResponse, setIsSuccessfulResponse] = useState<
    boolean | null
  >(null);

  const [browserWallets] = useState(
    allSubstrateWallets.filter((w) =>
      w.platforms.includes(SubstrateWalletPlatform.Browser)
    )
  );

  function getMessageToSign() {
    return `I want to login with Azero.GG to verify my identity: ${activeAccount?.address}`;
  }

  async function connectWallet(w: SubstrateWallet) {
    if (connect) {
      await connect(undefined, w);
      const address = activeAccount?.address;
      if (address) {
        console.log("Connected to wallet with address: ", address);
      } else {
        console.error("No address found in active account");
      }
    } else {
      console.error("Connect function not available");
    }
  }

  // If the wallet is not connected, show the menu with connection options
  if (!activeAccount) {
    return (
      <Menu>
        <h2 className="text-gray-300 text-lg mb-4">
          Please sign in with your wallet to verify your on-chain identity
        </h2>

        <MenuButton
          as={Button}
          isLoading={isConnecting}
          size="md"
          rightIcon={<FiChevronDown size={22} />}
          py={6}
          fontWeight="bold"
          rounded="xl"
          colorScheme="orange"
        >
          Connect Wallet
        </MenuButton>

        <MenuList
          bgColor="blackAlpha.500"
          borderColor="whiteAlpha.500"
          rounded="xl"
        >
          {/* Installed Wallets */}
          {!activeAccount &&
            browserWallets.map((w) =>
              isWalletInstalled(w) ? (
                <MenuItem
                  key={w.id}
                  onClick={() => connectWallet(w)}
                  tw="bg-transparent hocus:bg-gray-500"
                >
                  {w.name}
                </MenuItem>
              ) : (
                <MenuItem
                  as={Link}
                  href={w.urls.website}
                  key={w.id}
                  tw="bg-transparent opacity-50 hocus:bg-gray-600 hover:(no-underline opacity-70)"
                >
                  <VStack align="start" spacing={0}>
                    <HStack>
                      <Text>{w.name}</Text>
                      <FiExternalLink size={16} />
                    </HStack>
                    <Text fontSize="xs">Not installed</Text>
                  </VStack>
                </MenuItem>
              )
            )}
        </MenuList>
      </Menu>
    );
  }

  // Wallet is connected
  else {
    // Signature is required
    if (!hasSigned) {
      return (
        <div className="text-center">
          <h2 className="text-gray-300 text-lg mb-4">
            Please sign the message below:
          </h2>

          <h3
            className="text-gray-300 text-md mb-4 p-3 border border-gray-700 
                        rounded-xl max-w-lg"
          >
            {getMessageToSign()}
          </h3>

          <Button
            colorScheme="orange"
            onClick={async () => {
              setIsSigning(true);

              let payload: SignerPayloadRaw = {
                address: activeAccount?.address,
                data: getMessageToSign(),
                type: "bytes",
              };

              activeSigner
                ?.signRaw?.(payload)
                .then(async (sig) => {
                  setHasSigned(true);
                  setIsSigning(false);

                  // Send POST request to the service
                  const discordId = new URLSearchParams(
                    window.location.search
                  ).get("discordId");
                  const guildId = new URLSearchParams(
                    window.location.search
                  ).get("guildId");
                  if (!discordId || !guildId) {
                    console.error(
                      "discordId and guildId must be provided in the url"
                    );
                    return;
                  }

                  const body = {
                    guildId: guildId,
                    discordId: discordId,
                    accountId: activeAccount?.address,
                    signature: sig.signature,
                  };
                  console.log("BODY: ", body);

                  const res = await fetch("/auth", {
                    method: "POST",
                    headers: {
                      "Access-Control-Allow-Origin": "*",
                      "Content-Type": "application/json",
                      Accept: "application/json",
                    },
                    body: JSON.stringify(body),
                    // mode: "cors",
                  });

                  const response = await res.json();
                  console.log("RESPONSE: ", response);

                  if (response.message === "Requirements not satisfied.") {
                    setIsSuccessfulResponse(false);
                  }
                  if (response.message === "User authorized.") {
                    setIsSuccessfulResponse(true);
                  }
                })
                .catch((err) => {
                  console.error(err);
                });
            }}
          >
            Sign Message
          </Button>
        </div>
      );
    }

    // user has signed in, show the success message
    else {
      if (isSuccessfulResponse === true) {
        return (
          <div className="text-center">
            <div className="mt-4 mx-auto mb-6">
              <AiOutlineCheckCircle size={50} className="mx-auto" />
              <h2 className="text-2xl font-bold">Success!</h2>
              <p className="mt-2">
                You have successfully logged in with your wallet.
              </p>
            </div>
            <Button
              colorScheme="orange"
              onClick={() => {
                disconnect?.();
                setHasSigned(false);
              }}
            >
              <AiOutlineDisconnect size={20} />
              <span className="ml-2">Disconnect</span>
            </Button>
          </div>
        );
      } else if (isSuccessfulResponse === false) {
        return (
          <div className="text-center">
            <div className="mt-4 mx-auto mb-6">
              <AiOutlineCheckCircle size={50} className="mx-auto" />
              <h2 className="text-2xl font-bold">Sorry :(</h2>
              <p className="mt-2">
                It looks like you don't meet the requirements of this server.
              </p>
            </div>
            <Button
              colorScheme="orange"
              onClick={() => {
                disconnect?.();
                setHasSigned(false);
              }}
            >
              <AiOutlineDisconnect size={20} />
              <span className="ml-2">Disconnect</span>
            </Button>
          </div>
        );
      } else {
        return (
          <div className="text-center">
            <div className="mt-4 mx-auto mb-6">
              <AiOutlineCheckCircle size={50} className="mx-auto" />
              <h2 className="text-2xl font-bold">Success!</h2>
              <p className="mt-2">
                You have successfully logged in with your wallet.
              </p>
            </div>
            <Button
              colorScheme="orange"
              onClick={() => {
                disconnect?.();
                setHasSigned(false);
              }}
            >
              <AiOutlineDisconnect size={20} />
              <span className="ml-2">Disconnect</span>
            </Button>
          </div>
        );
      }
    }
  }
}
