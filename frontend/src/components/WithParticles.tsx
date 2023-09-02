import { useCallback } from "react";
import Particles from "react-tsparticles";
import { loadFull } from "tsparticles";

import type { Container, Engine } from "tsparticles-engine";

interface Props {
  children: JSX.Element;
  opts?: any;
}

export default function WithParticles({ children, opts }: Props) {
  const particlesInit = useCallback(
    async (engine: Engine) => await loadFull(engine),
    []
  );

  const particlesLoaded = useCallback(
    async (container: Container | undefined) => {},
    []
  );

  return (
    <>
      <Particles
        id="tsparticles"
        init={particlesInit}
        loaded={particlesLoaded}
        options={opts ?? (options as any)}
        className="fixed inset-0 -z-10"
      />

      <div className="z-10">{children}</div>
    </>
  );
}

const options = {
  fpsLimit: 60,
  background: { color: "transparent" },
  particles: {
    color: { value: "#20aea0" },
    move: {
      direction: "none",
      enable: true,
      outModes: "out",
      random: true,
      speed: 1.5,
      straight: false,
    },
    number: {
      density: {
        enable: true,
        area: 1000,
      },
      value: 175,
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