"use client";

import React, { useState, useEffect } from "react";
import axios from "axios";

export default async function DataFetching(endpoint) {
  const [data, setData] = useState([]);
  const [message, setMessage] = useState();

  const fetchdata = async () => {
    const response = await axios.get(`http://+${endpoint}`);

    if (response.status === 200) {
      setMessage("Fetching data successful");
      return response.data;
    } else {
      setMessage("Fetching data failed");
      return null;
    }
  };

  const apiHandler = fetchdata();
  if (apiHandler != null) {
    setData(apiHandler.data);
    return data;
  } else {
    return console.error("Something went wrong.");
  }
}
