import { useEffect, useState } from "react";
import { useCurrencyContext } from "../context/CurrencyContext";
import styled from "styled-components";
import { Link } from "react-router-dom";

const TabButton = styled.button<{ isCurrentTab?: boolean }>(
  ({ isCurrentTab }) => ({
    background: isCurrentTab ? "#ddd" : "white",
    margin: 0,
    borderRadius: 0,
    outline: 'none',
  })
);


const Wrapper = styled.div(() => ({
    marginLeft: "50px",
    marginTop: "50px",
  }));

const Table = styled.table(() => ({
  boxShadow: "1px 2px 5px 0px #aaa",
  borderRadius: "2px",
  marginBottom: "20px",
  padding: "5px",
  borderCollapse: "collapse",
  minWidth: "350px",

  "& th": {
    textAlign: "left",
    paddingLeft: "15px",
    fontFamily: "sans-serif",
    fontSize: "1.2rem",
    fontWeight: "500",
    textTransform: "uppercase",
  },

  "& tr": {
    borderTop: "1px solid #ddd",
    display: "flex",
    justifyContent: "space-between",
  },

  "& td": {
    padding: "5px 15px",
  },
}));

const Summary = () => {
  const [totalList, setTotalList] = useState<{ [name: string]: number }>({});
  const [exchangeList, setExchangeList] = useState({});

  const { currencies } = useCurrencyContext();

  useEffect(() => {
    const selectedCurrencies = Object.keys(currencies.dict).filter(
      (currencyCode) => currencies.dict[currencyCode].isSelected
    );

    // selectedCurrencies.forEach((currency) => setTotalList((prev) => ({...prev, [currency]: currencies.dict[currency].existingAmount || 0})));
    selectedCurrencies.forEach((currency) => {
      const currencyConvRate = currencies.dict[currency].conversionRate;
      selectedCurrencies.forEach((otherCurrency) => {
        if (currency !== otherCurrency) {
          const otherCurrencyConvRate =
            currencies.dict[otherCurrency].conversionRate;
          const currencyToOtherCurrency =
            currencyConvRate / otherCurrencyConvRate;
          const otherCurrencyToCurrency = 1 / currencyToOtherCurrency;
          setTotalList((prev) => ({
            ...prev,
            [otherCurrency]:
              (currencies.dict[otherCurrency].existingAmount || 0) +
              (currencies.dict[currency].existingAmount || 0) *
                otherCurrencyToCurrency,
            [currency]:
              (currencies.dict[currency].existingAmount || 0) +
              (currencies.dict[otherCurrency].existingAmount || 0) *
                currencyToOtherCurrency,
          }));

          setExchangeList({
            ...exchangeList,
            [otherCurrency + "/" + currency]: currencyToOtherCurrency,
            [currency + "/" + otherCurrency]: otherCurrencyToCurrency,
          });
        }
      });
    });
  }, []);

  return (
    <Wrapper>
      <div style={{marginBottom: "40px"}}>
        <Link to="/">
          <TabButton>Assets</TabButton>
        </Link>
        <Link to="/summary">
          <TabButton isCurrentTab>Summary</TabButton>
        </Link>
      </div>
      <Table>
        <th>Total</th>
        <tbody>
          {Object.entries(totalList).map(([key, total]) => (
            <tr>
              <td>{key}</td>
              <td>{total}</td>
            </tr>
          ))}
        </tbody>
      </Table>

      <Table>
        <th>Exchange</th>
        <tbody>
          {Object.entries(exchangeList).map(([key, rate]) => (
            <tr>
              <td>{key}</td>
              <td>{rate as string}</td>
            </tr>
          ))}
        </tbody>
      </Table>
    </Wrapper>
  );
};

export default Summary;
