import { useState } from 'react'
import { 
    Button, HStack, Link, Menu, MenuButton, 
    MenuDivider, MenuItem, MenuList, Text, VStack 
} from '@chakra-ui/react'
import {
    SubstrateChain, SubstrateWallet, SubstrateWalletPlatform, allSubstrateWallets, getSubstrateChain, 
    isWalletInstalled, useBalance, useInkathon
} from '@scio-labs/use-inkathon'
import { AiOutlineCheckCircle, AiOutlineDisconnect } from 'react-icons/ai'
import { FiChevronDown, FiExternalLink } from 'react-icons/fi'
import { SignerPayloadRaw } from '@polkadot/types/types'

export default function ConnectWalletButton() {
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
      } = useInkathon()

      const [hasSigned, setHasSigned] = useState(false)

      const [browserWallets] = useState(
        allSubstrateWallets.filter((w) => w.platforms.includes(SubstrateWalletPlatform.Browser)),
      )

      async function connectWallet(w: SubstrateWallet) {
        const discordId = new URLSearchParams(window.location.search).get('discordId')
        if (!discordId) {
            console.error('discordId must be provided in the url')
            return
        }

        if (connect) {
            await connect(undefined, w);
            const address = activeAccount?.address;
            if (address) {
                console.log('Connected to wallet with address: ', address)
                await fetch(`http://localhost:8080/auth/${discordId}/${address}`)
            } else {
                console.error('No address found in active account')
            }
        } else {
            console.error('Connect function not available')
        }
    }

    // If the wallet is not connected, show the menu with connection options
    if (!activeAccount) {
        return (
            <Menu>
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

                <MenuList bgColor="blackAlpha.500" borderColor="whiteAlpha.500" rounded="xl">
                {/* Installed Wallets */}
                {
                    !activeAccount &&
                    browserWallets.map((w) =>
                    isWalletInstalled(w) ? (
                        <MenuItem
                        key={w.id}
                        onClick={() => {
                            connectWallet(w)
                        }}
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
                    ),
                    )}
                </MenuList>
            </Menu>
        )
    }

    
    else {
        // Signature is required
        if (!hasSigned) {
            return (
                <div className="text-center">
                    {/* <Button colorScheme="orange" onClick={() => disconnect?.()}>Disconnect</Button> */}
                    <Button colorScheme="orange" onClick={async () => {
                        let payload: SignerPayloadRaw = {
                            address: activeAccount?.address,
                            data: "I want to login with Azero.GG to verify my identity!",
                            type: "bytes"
                        }

                        activeSigner?.signRaw?.(payload).then((sig) => {
                            console.log(sig)
                            setHasSigned(true)
                        }).catch((err) => {
                            console.error(err)
                        })
                    }}>
                        Sign Message
                    </Button>
                </div>
            )
        }

        // user has signed in, show the success message
        else {
            return (
                <div>

                </div>
            )
        }
    }
}
