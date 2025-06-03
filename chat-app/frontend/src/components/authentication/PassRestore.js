"use client";

import React from "react";
import axios from "axios";
import { useRouter } from "next/navigation";

function UpdatingPassword() {
  const [NewPass, setNewPass] = useState();
  const [Otp, setOtp] = useState();
  const fodmData = new ForData();

  const SubmitHandle = async (e) => {
    formData.append(sd);
    const response = await axios.post("http://127.0.0.1:8080/pass", {});

    if (response.status === 200) {
      alert(response.data.message);
      router
    }
  };
}
