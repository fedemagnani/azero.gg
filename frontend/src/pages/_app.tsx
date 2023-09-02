import '@/styles/globals.css'

import type { AppProps } from 'next/app'
import { Inter } from 'next/font/google'
import Head from 'next/head'

import { UseInkathonProvider } from '@scio-labs/use-inkathon'
import { ChakraProvider, DarkMode } from '@chakra-ui/react'

const inter = Inter({ subsets: ['latin'] })

export default function App({ Component, pageProps }: AppProps) {
  return (
    <div className={inter.className}>
      <Head>
        <meta name="viewport" content="initial-scale=1.0, width=device-width" />
      </Head>

      <UseInkathonProvider
            appName="azero.gg"
            connectOnInit={false}
            defaultChain="alephzero-testnet"
            // deployments={getDeployments()}
          >
            <ChakraProvider>
              <DarkMode>
                <Component {...pageProps} />
              </DarkMode>
            </ChakraProvider>
      </UseInkathonProvider>
    </div>
  )
}
