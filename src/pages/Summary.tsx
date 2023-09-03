import { useEffect, useState } from "react";
import { useCurrencyContext } from "../context/CurrencyContext";
import styled from "styled-components";
import Tabs from "../components/Tabs";

const Wrapper = styled.div(() => ({
  marginLeft: "50px",
  marginTop: "50px",
}));

const TablesWrapper = styled.div(() => ({
  display: "flex",
  flexWrap: "wrap",
}));

const Table = styled.table(() => ({
  boxShadow: "1px 2px 5px 0px #aaa",
  borderRadius: "2px",
  marginBottom: "30px",
  padding: "5px",
  borderCollapse: "collapse",
  minWidth: "350px",
  height: "max-content",
  marginRight: "30px",

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

    const total: { [name: string]: number } = {};
    const exchange: { [name: string]: number } = {};
    for (let currency of selectedCurrencies) {
      total[currency] = currencies.dict[currency].existingAmount || 0;
    }
    for (let currency of selectedCurrencies) {
      for (let otherCurrency of selectedCurrencies) {
        if (currency !== otherCurrency) {
          total[currency] =
            (total[currency] || 0) +
            (currencies.dict[otherCurrency].existingAmount || 0) *
              (currencies.dict[currency].conversionRate /
                currencies.dict[otherCurrency].conversionRate);

          exchange[`${currency}/${otherCurrency}`] =
            currencies.dict[otherCurrency].conversionRate /
            currencies.dict[currency].conversionRate;
          exchange[`${otherCurrency}/${currency}`] =
            currencies.dict[currency].conversionRate /
            currencies.dict[otherCurrency].conversionRate;
        }
      }
    }

    setTotalList({ ...total });
    setExchangeList({ ...exchange });
  }, []);

  return (
    <Wrapper>
      <Tabs
        data={[
          { name: "Assets", path: "/" },
          { name: "Summary", path: "/summary" },
        ]}
      />

      {Object.keys(totalList).length > 0 ? (
        <TablesWrapper>
          <Table>
            <thead>
              <tr>
                <th>Total</th>
              </tr>
            </thead>
            <tbody>
              {Object.entries(totalList).map(([key, total]) => (
                <tr key={key}>
                  <td>{key}</td>
                  <td>{total.toFixed(8)}</td>
                </tr>
              ))}
            </tbody>
          </Table>

          <Table>
            <thead>
              <tr>
                <th>Exchange</th>
              </tr>
            </thead>
            <tbody>
              {Object.entries(exchangeList).map(([key, rate]) => (
                <tr key={key}>
                  <td>{key}</td>
                  <td>{(rate as number).toFixed(8)}</td>
                </tr>
              ))}
            </tbody>
          </Table>
        </TablesWrapper>
      ) : (
        <span>Nothing to show. Add your assets to get started.</span>
      )}
    </Wrapper>
  );
};

export default Summary;
