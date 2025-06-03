"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import axios from "axios";
import Link from "next/link";

export default function Login() {
  const router = useRouter();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");

  const handleLogin = async (e) => {
    e.preventDefault();

    try {
      // Send login request to your backend API endpoint.
      const response = await axios.post("http://127.0.0.1:8080/login", {
        username,
        password,
      });

      if (response.status === 200) {
        // Optionally save the returned token.
        localStorage.setItem("token", response.data.token);
        // Redirect to the chat page or home dashboard.
        router.push("/chat");
      } else {
        setError("Login failed. Please check your credentials.");
      }
    } catch (err) {
      console.error("Login error:", err);
      setError("Login failed. Please check your credentials.");
    }
  };

  return (
    <div className="min-h-screen flex flex-col justify-center items-center bg-gray-100">
      <h1 className="text-4xl font-bold mb-8 text-gray-800">
        Welcome to ChatApp
      </h1>
      <form
        onSubmit={handleLogin}
        className="w-full max-w-sm bg-white p-8 rounded shadow-md"
      >
        {error && <p className="mb-4 text-center text-red-500">{error}</p>}
        <div className="mb-6">
          <label
            htmlFor="username"
            className="block text-gray-700 font-semibold mb-2"
          >
            Username
          </label>
          <input
            id="username"
            type="text"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            required
            className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring focus:ring-blue-300"
          />
        </div>
        <div className="mb-6">
          <label
            htmlFor="password"
            className="block text-gray-700 font-semibold mb-2"
          >
            Password
          </label>
          <input
            id="password"
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
            className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring focus:ring-blue-300"
          />
        </div>
        <div className="flex justify-center">
          <button
            type="submit"
            className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
          >
            Log In
          </button>
        </div>
        <div className="flex  justify-between mt-4">
          <div className="flex">
            <ul className="ContainerLinks">
              <li>
                <Link href="/signin/passrestore">Forgot password?</Link>
              </li>
            </ul>
          </div>
          <div className="flex">
            <ul className="ContainerLikns">
              <li>
                <Link href="/registration">Already have an account?</Link>
              </li>
            </ul>
          </div>
        </div>
      </form>
    </div>
  );
}
