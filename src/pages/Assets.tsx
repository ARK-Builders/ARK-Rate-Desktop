import { useState } from "react";
import { useCurrencyContext } from "../context/CurrencyContext";
import styled from "styled-components";
import CurrencyListPopup from "../components/CurrencyListPopup";
import DeleteIcon from "../assets/delete.svg";
import Tabs from "../components/Tabs";

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
          existingAmount: 0,
        },
      },
    });
  };

  const selectedCurrencies = Object.keys(currencies.dict).filter(
    (currency) => currencies.dict[currency].isSelected
  );

  return (
    <Wrapper>
      <Tabs
        data={[
          { name: "Assets", path: "/" },
          { name: "Summary", path: "/summary" },
        ]}
      />

      {selectedCurrencies.length > 0 ? (
        selectedCurrencies.map((currency) => (
          <CurrencyInputWrapper key={currency}>
            <input
              type="number"
              placeholder=" "
              value={currencies.dict[currency].existingAmount}
              onChange={(e) => handleCurrencyInput(e.target.value, currency)}
            />
            <label>{currency}</label>
            <DeleteBtn
              src={DeleteIcon}
              onClick={() => handleDeleteInput(currency)}
            />
          </CurrencyInputWrapper>
        ))
      ) : (
        <p>Start by adding your assets using the "Add +" button below.</p>
      )}

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
