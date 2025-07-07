"use client";

import React, { useState, useEffect } from "react";
import axios from "axios";
import { useRouter } from "next/navigation";
import Link from "next/link";

export default function UpdatingPassword() {
  const [NewPass, setNewPass] = useState();
  const [Otp, setOtp] = useState();
  const [phone, setPhone] = useState();
  const [userExist, setUserExist] = useState(false);
  const [loading, setLoading] = useState(false);

  const SubmitHandle = async (e) => {
    formData.append(sd);
    const response = await axios.post("http://127.0.0.1:8080/pass", {});

    if (response.status === 200) {
      alert(response.data.message);
      router;
    }
  };

  return (
    <div className="flex items-right justify-center min-w-screen bg-gray-100 min-h-screen">
      <div className="w-[%50] p-8 space-y-6 bg-white shadow-lg rounded-lg">
        <form className="w-[%50] text-2xl font-bold text-center text-gray-800">
          <input
            type="text"
            placeholder="Enter phone number or email address"
            value={phone}
            onChange={(e) => setPhone(e.target.value)}
            className="w-full px-4 py-2 border border-gray-300 rounded-md focues:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            className={`w-full px-4 py-2 text-white rounded-md ${loading || !userExist ? "bg-gray-400  cursor-not-allowed" : "bg-blue-500 hover hover:bg-blue-600"}`}
            type="submit"
          >
            {loading ? "Sending..." : "Send OTP"}
          </button>
          <Link href="/" className="text-blue-600 hover:underline p-4">
            Login
          </Link>
          <Link href="/signup" className="text-blue-600 hover:underline p-4">
            Register
          </Link>
        </form>
      </div>
    </div>
  );
}
