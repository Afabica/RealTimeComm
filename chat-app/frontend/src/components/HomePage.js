"use client"; // only if using App Router

import React, { useState, useEffect } from "react";
import Link from "next/link";

const HomePage = () => {
  return (
    <main className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100 px-4 text-center">
      <h1 className="text-4xl md:text-5xl font-bold text-indigo-700 mb-4">
        Welcome to ChatConnect
      </h1>
      <p className="text-lg md:text-xl text-gray-700 max-w-xl mb-8">
        ChatConnect is a modern real-time chat application that lets you connect
        and collaborate instantly with anyone, anywhere. Enjoy secure messaging,
        responsive design, and fast performance — all built with Next.js and
        React.
      </p>

      <div className="flex gap-4">
        <Link href="/register">
          <button className="px-6 py-2 bg-indigo-600 text-white rounded-xl hover:bg-indigo-700 transition">
            Get Started
          </button>
        </Link>
        <Link href="/signup">
          <button className="px-6 py-2 border border-indigo-600 text-indigo-600 rounded-xl hover:bg-indigo-50 transition">
            Log In
          </button>
        </Link>
      </div>
    </main>
  );
};

export default HomePage;
