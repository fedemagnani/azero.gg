import NoSSR from '@/components/NoSSR'
import WalletButton from '@/components/WalletButton'

export default function Home() {
  return (
    <main className="w-full min-h-screen bg-black text-gray-100">
        <div className="w-full grid place-content-center mt-48">
          <NoSSR>
              <h1 className="text-center text-6xl mb-4 tracking-widest font-bold">AZERO.GG</h1>
              
              <h2 className="text-gray-300 text-lg mb-4">
                Please sign in with your wallet to verify your on-chain identity
              </h2>

              <WalletButton />
          </NoSSR>
        </div>
    </main>
  )
}
