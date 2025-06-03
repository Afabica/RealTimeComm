"use client";

import React, { useState, useEffect } from "react";
import axios from "axios";
import dynamic from "next/dynamic";
import franc from "franc-min";

function LanguageSelector(friends) {
  const [text, setText] = useState();
  const [detectedLanguage, setDetectedLanguage] = useState();
  const [translation, setTranslation] = useState();

  const handleInputChange = () => {
    const inputMessage = e.target.value;
    setText(inputMessage);

    const language = franc(text);
    if (language !== "und") {
      setDetectedLanguage(language);
    } else {
      setDetectedLanguage(null);
    }
  };

  

  return <div></div>;
}

export default LanguageSelector;
