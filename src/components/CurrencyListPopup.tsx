import { useEffect, useState } from "react";
import closeIcon from "../assets/close.svg";
import styled from "styled-components";

const CloseIcon = styled.img(() => ({
  height: "20px",
  marginLeft: "96%",
  marginTop: "10px",
  top: "10px",
  cursor: "pointer",
}));

const PopupWrapper = styled.section(() => ({
  backgroundColor: "white",
  borderRadius: "2px",
  position: "absolute",
  top: "0px",
  left: "0px",
  height: "100vh",
  overflowY: "hidden",
  width: "100vw",
}));

const ListWrapper = styled.section(() => ({
  overflowY: "scroll",
  maxHeight: "80vh",
}));

const CurrencyItem = styled.p(() => ({
  color: "black",
  marginBottom: "0px",
  marginTop: "0px",
  padding: "10px 20px",
  borderBottom: "1px solid #ddd",
  transition: "background-color 200ms ease",
  cursor: "pointer",

  "&:hover": {
    backgroundColor: "#ddd",
  },
}));

const SearchBar = styled.input(() => ({
  padding: "10px",
  outline: "none",
  marginLeft: "20px",
  marginBottom: "15px",
  backgroundColor: "transparent",
  border: "1px solid #ddd",
  color: "black",
}));

const CurrencyListPopup = ({
  onClose,
  onSelectCurrency,
  currencies,
}: {
  onClose: () => void;
  onSelectCurrency: (currency: string) => void;
  currencies: string[];
}) => {
  const [currencyList, setCurrencyList] = useState(currencies);
  const [searchText, setSearchText] = useState<string>("");

  useEffect(() => {
    searchText &&
      setCurrencyList(currencies.filter((value) => value.toLowerCase().includes(searchText.toLowerCase())));
  }, [searchText]);

  return (
    <PopupWrapper>
      <CloseIcon onClick={onClose} src={closeIcon} />
      <SearchBar
        placeholder="Search"
        onChange={(e) => setSearchText(e.target.value)}
        autoFocus
      />
      <ListWrapper>
        {currencyList.map((currency) => (
          <CurrencyItem
            key={currency}
            onClick={() => {
              onSelectCurrency(currency);
              onClose();
            }}
          >
            {currency}
          </CurrencyItem>
        ))}
      </ListWrapper>
    </PopupWrapper>
  );
};

export default CurrencyListPopup;
