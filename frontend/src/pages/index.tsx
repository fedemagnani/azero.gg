import Image from "next/image"

import NoSSR from '@/components/NoSSR'
import WalletButton from '@/components/WalletButton'
import WithParticles from "@/components/WithParticles";
import { useState } from "react";

const opts = (hovered: boolean) => {
  return {
    fpsLimit: 90,
    background: { color: "transparent" },
    particles: {
      color: { value: "#FFFFFF" },
      move: {
        direction: "none",
        enable: true,
        outModes: "out",
        random: false,
        speed: hovered ? 22 : 5,
        straight: false,
      },
      number: {
        density: {
          enable: true,
          area: 900,
        },
        value: 300,
      },
      opacity: {
        animation: {
          enable: false,
          speed: 0.05,
          sync: true,
          startValue: "max",
          count: 1,
          destroy: "min",
        },
        value: {
          min: 0,
          max: 0.5,
        },
      },
      shape: {
        type: "circle",
      },
      size: {
        value: { min: 0.3, max: 3.4 },
      },
    },
  };
};

export default function Home() {
  const [isSigning, setIsSigning] = useState(false)

  return (
    <main className="w-full bg-black text-gray-100">
        <div className="w-full grid place-content-center mt-72">
          <NoSSR>
            <WithParticles opts={opts(isSigning)}>
              <div className="text-center">
                <h1 className="text-center text-7xl mb-4 tracking-widest font-bold">
                  AZERO.GG
                </h1>

                <WalletButton setIsSigning={setIsSigning} />
              </div>
              </WithParticles>
          </NoSSR>
        </div>
    </main>
  )
}
