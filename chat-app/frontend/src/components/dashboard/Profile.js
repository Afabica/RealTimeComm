"use client";

import React, { useState, useEffect } from "react";
import Header from "../hedfot/HomeHeader";
import axios from "axios";
import dynamic from "next/dynamic";
import SidePanel from "./SidePanel";

const SidePanel = dynamic(() => import("./SidePanel"), {
  ssr: false,
});

const ProfilePage = () => {
  <div className="flex max-h-screen bg-gray-100 text-gray-900"></div>;
};

export default ProfilePage;
