/**
 * Definition for Employee.
 * function Employee(id, importance, subordinates) {
 *     this.id = id;
 *     this.importance = importance;
 *     this.subordinates = subordinates;
 * }
 */

const VALUE_INDEX = 1;
const SUBS_INDEX = 2;

/**
 * @param {Employee[]} employees
 * @param {number} id
 * @return {number}
 */
var GetImportance = function (employees, id) {
  return getScore(employees, id, []);
};

const getScore = (employees, id, seenId) => {
  const index = employees.findIndex((e) => e.id === id);
  const employee = employees[index];
  if (employee.subordinates.length > 0) {
    let score = employees.subordinates.reduce((employee, currentV) => {
      return (
        currentV +
        getScore(employees, e.id, [...seenId, ...employee.subordinates])
      );
    }, employee.importance);
    return score;
  } else if (seenId.contains(id)) {
    return 0;
  } else {
    return employee.importance;
  }
};

const selectedAllCovid = () => {
  var radioElements = document.getElementsByClassName("QWatchTimer");
  for (var i = 0; i < radioElements.length; i++) {
    radioElements[i].checked = true;
  }
};

selectedAllCovid();
