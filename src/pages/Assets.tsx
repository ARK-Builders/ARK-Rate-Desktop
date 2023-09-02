import { useState } from "react";
import { useCurrencyContext } from "../context/CurrencyContext";
import styled from "styled-components";
import CurrencyListPopup from "../components/CurrencyListPopup";
import { Link } from "react-router-dom";
import DeleteIcon from "../assets/delete.svg";

const TabButton = styled.button<{ isCurrentTab?: boolean }>(
  ({ isCurrentTab }) => ({
    background: isCurrentTab ? "#ddd" : "white",
    margin: 0,
    borderRadius: 0,
    outline: "none",
  })
);

const Wrapper = styled.div(() => ({
  marginLeft: "50px",
  marginTop: "50px",
}));

const CurrencyInputWrapper = styled.div(() => ({
  display: "flex",
  alignItems: "center",
  marginBottom: "20px",
  position: "relative",

  "& > input": {
    boxShadow: "none",
    border: "2px solid #ddd",
  },

  "& > input::placeholder": {
    color: "transparent",
  },

  "& > label": {
    position: "absolute",
    left: "13px",
    bottom: "12px",
    color: "#999",
    transition: "bottom 0.15s ease, font-size 0.15s ease",
    background: "white",
  },

  "& > input:focus ~ label": {
    fontSize: "0.8rem",
    bottom: "35px",
  },

  "& > input:not(:placeholder-shown) ~ label": {
    fontSize: "0.8rem",
    bottom: "35px",
  },
}));

const DeleteBtn = styled.img(() => ({
  cursor: "pointer",
  marginLeft: "10px",
}));

const Assets = () => {
  const [showList, toggleListState] = useState(false);
  const { currencies, setCurrencies } = useCurrencyContext();

  const handleCurrencyInput = (value: string, currencyCode: string) => {
    setCurrencies({
      ...currencies,
      dict: {
        ...currencies.dict,
        [currencyCode]: {
          ...currencies.dict[currencyCode],
          existingAmount: parseFloat(value),
        },
      },
    });
  };

  const handleDeleteInput = (currencyCode: string) => {
    setCurrencies({
      ...currencies,
      dict: {
        ...currencies.dict,
        [currencyCode]: {
          ...currencies.dict[currencyCode],
          isSelected: false,
        },
      },
    });
  };

  return (
    <Wrapper>
      <div style={{ marginBottom: "40px" }}>
        <Link to="/">
          <TabButton isCurrentTab={true}>Assets</TabButton>
        </Link>
        <Link to="/summary">
          <TabButton>Summary</TabButton>
        </Link>
      </div>

      {Object.keys(currencies.dict)
        .filter((currency) => currencies.dict[currency].isSelected)
        .map((currency) => (
          <CurrencyInputWrapper>
            <input
              type="number"
              placeholder=" "
              onChange={(e) => handleCurrencyInput(e.target.value, currency)}
            />
            <label>{currency}</label>
            <DeleteBtn
              src={DeleteIcon}
              onClick={() => handleDeleteInput(currency)}
            />
          </CurrencyInputWrapper>
        ))}

      <button onClick={() => toggleListState(true)}>Add +</button>

      {showList && (
        <CurrencyListPopup
          onClose={() => toggleListState(false)}
          onSelectCurrency={(currency) => {
            setCurrencies({
              ...currencies,
              dict: {
                ...currencies.dict,
                [currency]: {
                  ...currencies.dict[currency],
                  isSelected: true,
                },
              },
            });
          }}
          currencies={Object.keys(currencies.dict)}
        />
      )}
    </Wrapper>
  );
};

export default Assets;
