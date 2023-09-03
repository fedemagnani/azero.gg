module.exports = () => {
  const rewrites = () => {
    return [
      {
        source: "/auth",
        destination: "http://127.0.0.1:8080/auth",
      },
    ];
  };
  return {
    reactStrictMode: true,
    rewrites,
  };
};
