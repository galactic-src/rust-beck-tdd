

class TestCase:
    def __init__(self, name):
        self.name = name

    def set_up(self):
        pass

    def run(self):
        self.set_up()
        method = getattr(self, self.name)
        method()


class WasRun(TestCase):
    def __init__(self, name):
        self.wasRun = None
        self.log = None
        TestCase.__init__(self, name)

    def set_up(self):
        self.wasRun = None
        self.log = "set_up "

    def test_method(self):
        self.wasRun = 1
        self.log += "test_method "


class TestCaseTest(TestCase):
    def set_up(self):
        self.test = WasRun("test_method")

    def test_running(self):
        self.test.run()
        assert self.test.wasRun

    def test_setup(self):
        self.test.run()
        assert ("set_up test_method " == self.test.log)

    def test_template_method(self):
        test = WasRun("test_method")
        test.run()
        assert ("set_up test_method tear_down " == test.log)


TestCaseTest("test_running").run()
TestCaseTest("test_setup").run()
TestCaseTest("test_template_method").run()
