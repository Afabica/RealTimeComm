"use client";

import { motion } from "framer-motion";
import { useState, useEffect } from "react";
import clsx from "clsx";

export default function AuthPage() {
  const [isLogin, setIsLogin] = useState(true);
  const [formData, setFormData] = useState({
    username: "",
    email: "",
    password: "",
  });

  const formVariants = {
    hidden: { opacity: 0, y: 40 },
    visible: {
      opacity: 1,
      y: 0,
      transition: { type: "spring", stiffness: 100, damping: 12 },
    },
  };

  const handleChange = async (e) => {
    const { name, value } = e.target;
    setFormData((prevData) => ({
      ...prevData,
      [name]: value,
    }));
  };

  useEffect(() => {
    setFormData({
      username: "",
      email: "",
      password: "",
    });
  }, [isLogin]);
  return (
    <div className="min-h-screen bg-gradient-to-br from-indigo-900 via-purple-900 to-pink-900 flex items-center justify-center relative overflow-hidden">
      {/* Chat bubbles animation background */}
      <div className="absolute w-full h-full overflow-hidden z-0">
        <motion.div
          animate={{ y: [-10, 10, -10] }}
          transition={{ repeat: Infinity, duration: 5, ease: "easeInOut" }}
          className="absolute left-10 top-10 w-40 h-40 bg-white/10 rounded-full blur-3xl"
        />
        <motion.div
          animate={{ x: [-15, 15, -15] }}
          transition={{ repeat: Infinity, duration: 7, ease: "easeInOut" }}
          className="absolute right-10 bottom-10 w-60 h-60 bg-white/5 rounded-full blur-2xl"
        />
      </div>

      {/* Auth card */}
      <motion.div
        initial="hidden"
        animate="visible"
        variants={formVariants}
        className="z-10 bg-white/10 border border-white/20 backdrop-blur-md text-white p-8 rounded-2xl shadow-2xl w-full max-w-md"
      >
        <h2 className="text-3xl font-semibold mb-6 text-center">
          {isLogin ? "Welcome Back 👋" : "Join the Chat 💬"}
        </h2>

        <form className="space-y-4">
          {!isLogin && (
            <motion.div variants={formVariants}>
              <input
                type="text"
                name="username" // ← Add this
                placeholder="Username"
                value={formData.username}
                onChange={handleChange}
                className="w-full p-3 bg-white/20 rounded-lg placeholder-white/60 focus:outline-none"
                required
              />
            </motion.div>
          )}

          <motion.div variants={formVariants}>
            <input
              type="email"
              name="email" // ← Add this
              placeholder="Email"
              value={formData.email}
              onChange={handleChange}
              className="w-full p-3 bg-white/20 rounded-lg placeholder-white/60 focus:outline-none"
              required
            />
          </motion.div>

          <motion.div variants={formVariants}>
            <input
              type="password"
              name="password" // ← Add this
              placeholder="Password"
              onChange={handleChange}
              value={formData.password}
              className="w-full p-3 bg-white/20 rounded-lg placeholder-white/60 focus:outline-none"
              required
            />
          </motion.div>

          {/* Chat-like loading dots (optional) */}
          {isLogin ? (
            <motion.div
              className="flex justify-center mt-6"
              onChange={handleChange}
              animate={{ opacity: [0.3, 1, 0.3] }}
              transition={{ repeat: Infinity, duration: 1.5 }}
            >
              <span className="text-sm text-white/70">🔐 Authenticating</span>
            </motion.div>
          ) : (
            <motion.div
              className="flex justify-center mt-6"
              animate={{ opacity: [0.3, 1, 0.3] }}
              transition={{ repeat: Infinity, duration: 1.5 }}
            >
              <span className="text-sm text-white/70">Registration</span>
            </motion.div>
          )}

          <motion.button
            whileHover={{ scale: 1.03 }}
            whileTap={{ scale: 0.98 }}
            className="w-full bg-indigo-600 hover:bg-indigo-700 transition text-white p-3 rounded-lg font-medium mt-4"
          >
            {isLogin ? "Login" : "Sign Up"}
          </motion.button>
        </form>

        <div className="text-center mt-4 text-sm">
          {isLogin ? (
            <>
              Don’t have an account?{" "}
              <button
                onClick={() => setIsLogin(false)}
                className="text-purple-300 hover:text-purple-200 underline"
              >
                Sign up
              </button>
            </>
          ) : (
            <>
              Already a user?{" "}
              <button
                onClick={() => setIsLogin(true)}
                className="text-purple-300 hover:text-purple-200 underline"
              >
                Login
              </button>
            </>
          )}
        </div>
      </motion.div>
    </div>
  );
}
