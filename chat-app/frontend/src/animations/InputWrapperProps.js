//"use client";
//
//import React from "react";
//import PropTypes from "prop-types";
//
//export function InputWrapper({ children, label }) {
//  return (
//    <div className={`flex flex-col mb-4 ${className || ""}`}>
//      {label && (
//        <label
//          className={`mb-1 font-medium text-gray-700 ${labelClassName || ""}`}
//        >
//          {label}
//        </label>
//      )}
//      {children}
//      {error && (
//        <span className={`mt-1 text-sm text-red-600 ${errorClassName || ""}`}>
//          {error}
//        </span>
//      )}
//    </div>
//  );
//}
//
//InputWrapper.propTypes = {
//  label: PropTypes.string,
//  error: PropTypes.string,
//  children: PropTypes.node.isRequired,
//  className: PropTypes.string,
//  labelClassName: PropTypes.string,
//  errorClassName: PropTypes.string,
//};
//
//InputWrapper.defaultProps = {
//  label: "",
//  error: null,
//  className: "",
//  labelClassName: "",
//  errorClassName: "",
//};
//
"use client";
import React from "react";

export default function InputWrapper({ children, label }) {
  return (
    <div className="mb-4 flex flex-col">
      <label className="block text-sm font-semibold text-gray-799 mb-1">
        {label}
      </label>
      {children}
    </div>
  );
}
